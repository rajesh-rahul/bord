use bord_sqlite3_parser::ungram::{
    Rule, UngramTraverser, UngramTraverserBacktrackResult, UngramTraverserNodeKind, UNGRAMMAR,
};
use bord_sqlite3_parser::{CstNodeData, CstNodeDataKind, CstNodeTrait, CstTrait, SqliteTokenKind};
use hashbrown::HashSet;
use itertools::Itertools;
use line_index::TextSize;

pub(crate) fn create_completion_context<Cst: CstTrait>(cst: &Cst, cursor: TextSize) -> Vec<String> {
    //// Find the token to the left of autocomplete position. We ignore trivial tokens (like whitespace tokens)
    let mut nodes_iter = cst.root().me_and_descendants().rev();

    let Some(mut target) = nodes_iter.find(|it| match it.token() {
        Some(_) => it.end_pos() <= cursor.into(),
        None => false,
    }) else {
        return Vec::new();
    };

    // If the token immediately to the left of the cursor is a trivial token, it is important
    // for us to know this but we keep going to find a target that is not trivial
    let ends_in_whitespace = target.token_kind() == Some(SqliteTokenKind::WHITESPACE);

    if ends_in_whitespace {
        let Some(t) = nodes_iter.find(|it| it.token().is_some_and(|it| !it.is_trivia())) else {
            return Vec::new();
        };

        target = t;
    }

    let path_to_target = calculate_path_to_target(&target);
    // tracing::info!(?path_to_target);

    let prefix_removed = |completions: &[String], prefix: &str| {
        completions
            .iter()
            .map(|it| it.to_string())
            .filter(|it| it.starts_with(prefix))
            .map(|it| it.trim_start_matches(prefix).to_string())
            .collect()
    };

    // Check for special cases which are handled in this match statement
    // The rest are handled by walking the grammar tree
    use AstPath::*;

    // TODO: Optimize
    match path_to_target.as_slice() {
        // [Me(";"), .., Ancestor("File")] | [Me("IDEN"), Ancestor("ErrorTree"), Ancestor("File")] => {
        //     return Some(find_completions(UNGRAMMAR.root()));
        // }
        [Me("CREATE"), ..] | [_, Sibling("CREATE"), ..] => {
            let completions = find_completions(UNGRAMMAR.root());
            return prefix_removed(&completions, "CREATE ");
        }
        [Me("DROP"), ..] | [_, Sibling("DROP"), ..] => {
            let completions = find_completions(UNGRAMMAR.root());
            return prefix_removed(&completions, "DROP ");
        }
        _ => {}
    }

    // If we didn't hit any special cases, we are ready to walk the grammar tree
    let mut traverser = UngramTraverser::new(cst.root(), UNGRAMMAR.root());
    let ancestors_ids: HashSet<<Cst::Node<'_> as CstNodeTrait<'_>>::Id> =
        target.ancestors().map(|it| it.id()).collect();

    let mut target_rule = None;

    while let Some(result) = traverser.next() {
        match result {
            UngramTraverserNodeKind::Token {
                name,
                ast_node,
                rule,
            } => match ast_node.map(|it| (it, it.data())) {
                Some((
                    node,
                    CstNodeData {
                        kind: CstNodeDataKind::Token(_),
                        ..
                    },
                )) if node.as_str() == name.trim_start_matches("KW_")
                    || node.as_str() == "IDEN" =>
                {
                    if node.equals(&target) {
                        target_rule = Some(rule);
                        break;
                    }
                    traverser.token_visited();
                }
                _ => {
                    if traverser.backtrack() == UngramTraverserBacktrackResult::Fail {
                        tracing::error!("Ungrammar and AST mismatch encountered");
                        return Vec::new();
                    }
                }
            },
            UngramTraverserNodeKind::Tree {
                name,
                rule,
                ast_node,
            } => match ast_node.map(|it| (it, it.data())) {
                Some((
                    node,
                    CstNodeData {
                        kind: CstNodeDataKind::Tree(_),
                        ..
                    },
                )) if node.as_str() == name => {
                    if node.equals(&target) {
                        target_rule = Some(rule);
                        break;
                    }

                    if ancestors_ids.contains(&node.id()) {
                        traverser.node_visited_and_expand_children();
                    } else {
                        traverser.node_visited();
                    }
                }
                _ => {
                    if traverser.backtrack() == UngramTraverserBacktrackResult::Fail {
                        tracing::error!("Ungrammar and AST mismatch encountered");
                        return Vec::new();
                    }
                }
            },
        }
    }

    match target_rule {
        Some(node) if traverser.rules_history().len() >= 2 => {
            let mut before_comp_target = node;
            let mut completions = CompletionNode::new_tree();

            loop {
                if let Some(comp_target) =
                    first_follow_rule(&before_comp_target, traverser.rules_history())
                {
                    // tracing::info!("completion_target: {}", rule_to_str(comp_target));

                    let mut new_tree = CompletionNode::new_tree();
                    make_completions(comp_target, &mut HashSet::new(), &mut new_tree, 0);
                    if matches!(before_comp_target, Rule::Token(_)) {
                        completions.append_tree_to_all_paths2(new_tree);
                    } else {
                        completions.add_new_path_child(new_tree);
                    }

                    let mut test_node = comp_target;
                    while let Rule::Labeled { rule, .. } = &test_node {
                        test_node = rule;
                    }

                    match &test_node {
                        Rule::Opt(_) | Rule::Rep(_) => before_comp_target = comp_target,
                        Rule::Token(tk)
                            if UNGRAMMAR.get_token(*tk).starts_with(char::is_alphabetic) =>
                        {
                            before_comp_target = comp_target
                        }
                        _ => break,
                    }
                } else {
                    tracing::warn!("No completion target found");
                    break;
                }
            }

            let mut all_paths = Vec::new();
            resolve_tree(&completions, &mut Vec::new(), &mut all_paths);

            let all_paths = all_paths
                .into_iter()
                .unique()
                .map(|str_vec| str_vec.join(" "))
                .collect();

            return all_paths;
        }
        _ => tracing::warn!("No completion target found"),
    }

    Vec::new()
}

fn find_completions(rule: &Rule) -> Vec<String> {
    let mut comp_node = CompletionNode::new_tree();
    make_completions(rule, &mut HashSet::new(), &mut comp_node, 0);

    let mut completions = Vec::new();
    resolve_tree(&comp_node, &mut Vec::new(), &mut completions);

    completions
        .into_iter()
        .unique()
        .map(|it| it.join(" "))
        .collect()
}

// TODO: return the parent idx so that subsequent calls to first_follow_rule can use the correct
// parents list
fn first_follow_rule<'a>(source: &'a Rule, parents: &[&'a Rule]) -> Option<&'a Rule> {
    let mut source = source;

    let rule_equals_source =
        |rule: &'a Rule, source: &'a Rule| rule as *const _ == source as *const _;

    for parent in parents.iter().rev() {
        match &parent {
            Rule::Seq(rules) => {
                let mut rules_iter = rules.iter();

                // If we find source in parent's children list, then we take its right sibling
                // as the first follow rule. If we do not have a right sibling, then we look for
                // the parent's right sibling
                if rules_iter
                    .find(|&it| rule_equals_source(it, source))
                    .is_some()
                {
                    match rules_iter.next() {
                        Some(rule) => return Some(rule),
                        _ => source = parent,
                    }
                };
            }
            Rule::Labeled { rule, .. } if rule_equals_source(rule, source) => {
                source = parent;
            }
            Rule::Node(node_id)
                if rule_equals_source(&UNGRAMMAR.get_node(*node_id).rule, source) =>
            {
                source = parent;
            }
            Rule::Alt(vec) => {
                if vec.iter().any(|it| rule_equals_source(it, source)) {
                    source = parent;
                }
            }
            Rule::Opt(rule) if rule_equals_source(rule, source) => {
                source = parent;
            }
            Rule::Rep(rule) if rule_equals_source(rule, source) => {
                return Some(parent);
            }
            _ => {}
        }
    }

    return None;
}

#[derive(Debug)]
pub enum AstPath {
    Me(&'static str),
    Sibling(&'static str),
    Ancestor(&'static str),
}

fn calculate_path_to_target<'a>(target: &impl CstNodeTrait<'a>) -> Vec<AstPath> {
    use AstPath::*;
    // Add my ancestors
    let mut path_to_target: Vec<_> = target
        .ancestors()
        .map(|ancestor| Ancestor(ancestor.as_str()))
        .collect();

    path_to_target.reverse(); // We need File at the beginning and target at the end

    let older_siblings = target.left_siblings().map(|it| Sibling(it.as_str()));
    path_to_target.extend(older_siblings);
    path_to_target.push(Me(target.as_str()));

    path_to_target.reverse();

    path_to_target
}

#[derive(Debug, Clone)]
struct CompletionNode {
    kind: CompletionNodeKind,
    children: Vec<CompletionNode>,
    is_fused: bool,
    allow_new_paths: bool,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum CompletionNodeKind {
    Root,
    Empty,
    Token(String),
}

impl CompletionNode {
    pub fn new_tree() -> Self {
        CompletionNode {
            kind: CompletionNodeKind::Root,
            children: Vec::new(),
            is_fused: false,
            allow_new_paths: false,
        }
    }

    pub fn add_optional(&mut self, mut optional_tree: CompletionNode) {
        assert!(optional_tree.kind == CompletionNodeKind::Root);

        if self.is_leaf() && !self.is_fused {
            self.children.append(&mut optional_tree.children);
            self.allow_new_paths = true;
        } else {
            for children in &mut self.children {
                children.add_optional(optional_tree.clone());
            }
        }
    }

    pub fn add_token_child(&mut self, token: String) {
        if self.is_leaf() && !self.is_fused {
            self.children.push(CompletionNode {
                kind: CompletionNodeKind::Token(token),
                children: Vec::new(),
                is_fused: false,
                allow_new_paths: false,
            });
        } else {
            for child in &mut self.children {
                child.add_token_child(token.clone());
            }
        }
    }

    pub fn fuse_nodes(&mut self) {
        if !self.is_new_path_addable() {
            self.is_fused = true;

            for child in &mut self.children {
                child.fuse_nodes();
            }
        }
    }

    pub fn append_tree_to_all_paths(&mut self, tree: CompletionNode) {
        assert!(tree.kind == CompletionNodeKind::Root);

        if !self.is_fused {
            self.combine_trees(tree);
        } else {
            for child in &mut self.children {
                child.append_tree_to_all_paths(tree.clone());
            }
        }
    }

    pub fn append_tree_to_all_paths2(&mut self, tree: CompletionNode) {
        assert!(tree.kind == CompletionNodeKind::Root);

        if self.is_leaf() && !self.is_fused {
            self.combine_trees(tree);
        } else {
            for child in &mut self.children {
                child.append_tree_to_all_paths(tree.clone());
            }
        }
    }

    pub fn add_empty_child(&mut self) {
        if self.kind != CompletionNodeKind::Root
            && self.is_leaf()
            && !self
                .children
                .iter()
                .any(|it| it.kind == CompletionNodeKind::Empty)
            && !self.is_fused
        {
            self.children.push(CompletionNode {
                kind: CompletionNodeKind::Empty,
                children: Vec::new(),
                is_fused: true,
                allow_new_paths: false,
            });
        }

        for child in &mut self.children {
            child.add_empty_child();
        }
    }
    pub fn combine_trees(&mut self, mut tree: CompletionNode) {
        assert!(tree.kind == CompletionNodeKind::Root);

        self.children.append(&mut tree.children);
        self.allow_new_paths = self.allow_new_paths || tree.allow_new_paths;
    }

    pub fn add_new_path_child(&mut self, tree: CompletionNode) {
        assert!(tree.kind == CompletionNodeKind::Root);

        if self.is_new_path_addable() {
            self.combine_trees(tree);
        } else {
            for child in &mut self.children {
                child.add_new_path_child(tree.clone());
            }
        }
    }

    pub fn can_add_new_path(&self) -> bool {
        if self.is_new_path_addable() {
            true
        } else {
            self.children.iter().any(|it| it.can_add_new_path())
        }
    }

    pub fn is_new_path_addable(&self) -> bool {
        self.allow_new_paths || self.is_leaf() && self.kind == CompletionNodeKind::Root
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
            || self.children.len() == 1 && self.children[0].kind == CompletionNodeKind::Empty
    }
}

fn resolve_tree(
    node: &CompletionNode,
    current_path: &mut Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    match &node.kind {
        CompletionNodeKind::Root => {
            if node.children.is_empty() && !current_path.is_empty() {
                all_paths.push(current_path.clone());
            }
            for child in &node.children {
                resolve_tree(child, current_path, all_paths);
            }
        }
        CompletionNodeKind::Empty => {
            if !current_path.is_empty() {
                all_paths.push(current_path.clone());
            }
        }
        CompletionNodeKind::Token(tk) => {
            current_path.push(tk.trim_start_matches("KW_").to_owned());
            if node.children.is_empty() {
                all_paths.push(current_path.clone());
            }
            for child in &node.children {
                resolve_tree(child, current_path, all_paths);
            }
            current_path.pop();
        }
    }
}

fn make_completions(
    rule: &Rule,
    seen_nodes: &mut HashSet<String>,
    completions: &mut CompletionNode,
    level: usize,
) -> bool {
    // eprintln!(
    //     "{}make_completions: {:?} - {completions:?}",
    //     "   ".repeat(level),
    //     rule_to_str(rule),
    // );

    let mut result;

    match &rule {
        Rule::Node(node_id) => {
            let node_data = UNGRAMMAR.get_node(*node_id);
            result = false;
            if !seen_nodes.contains(&node_data.name) {
                seen_nodes.insert(node_data.name.clone());

                if ["Expr", "TableOrSubquery", "JoinClause"].contains(&node_data.name.as_str()) {
                    completions.fuse_nodes();
                } else if completions.can_add_new_path() {
                    let mut new_completions = CompletionNode::new_tree();
                    result = make_completions(
                        &node_data.rule,
                        seen_nodes,
                        &mut new_completions,
                        level + 1,
                    );
                    if !result {
                        new_completions.fuse_nodes();
                    }
                    if !new_completions.is_leaf() {
                        completions.add_new_path_child(new_completions);
                    }
                } else {
                    completions.fuse_nodes();
                }
            } else {
                completions.fuse_nodes();
            }
        }
        Rule::Token(token_id) => {
            let name = UNGRAMMAR.get_token(*token_id);
            if name.chars().next().is_some_and(|it| !it.is_alphabetic()) {
                completions.fuse_nodes();
                result = false;
            } else {
                completions.add_token_child(name.to_owned());
                result = true;
            }
        }
        Rule::Labeled { rule, .. } => {
            result = make_completions(rule, seen_nodes, completions, level + 1)
        }
        Rule::Seq(vec) => {
            result = true;
            for rule in vec {
                if !make_completions(rule, seen_nodes, completions, level + 1) {
                    completions.fuse_nodes();
                    result = false;
                    break;
                }
            }
        }
        Rule::Alt(vec) => {
            let mut comp_tree = CompletionNode::new_tree();

            let mut overall_result = true;

            for rule in vec {
                let mut new_comp_tree = CompletionNode::new_tree();
                let new_result = make_completions(rule, seen_nodes, &mut new_comp_tree, level + 1);
                overall_result = overall_result && new_result;

                if !new_result {
                    new_comp_tree.fuse_nodes();
                }
                comp_tree.combine_trees(new_comp_tree);
            }
            completions.append_tree_to_all_paths(comp_tree);

            result = true;
        }
        Rule::Opt(rule) => {
            if completions.can_add_new_path() {
                let mut optional_tree = CompletionNode::new_tree();
                let res = make_completions(rule, seen_nodes, &mut optional_tree, level + 1);
                if !res {
                    optional_tree.fuse_nodes();
                }
                if !completions.is_leaf() || !optional_tree.is_leaf() {
                    completions.add_optional(optional_tree);
                }

                result = true;
            } else if matches!(rule.as_ref(), Rule::Node(_)) {
                result = false;
            } else {
                completions.add_empty_child();
                result = true;
            }
        }
        Rule::Rep(rule) => {
            if completions.can_add_new_path() {
                let mut optional_tree = CompletionNode::new_tree();
                let result = make_completions(rule, seen_nodes, &mut optional_tree, level + 1);
                if !result {
                    optional_tree.fuse_nodes();
                }
                completions.add_optional(optional_tree);
            }
            result = true;
        }
    }

    let mut all_paths = Vec::new();
    resolve_tree(completions, &mut Vec::new(), &mut all_paths);

    // eprintln!(
    //     "{}make_completions_done: {:?} - {completions:?}",
    //     "   ".repeat(level),
    //     rule_to_str(rule),
    // );

    return result;
}

#[cfg(test)]
mod completions_tests {
    use super::*;
    use bord_sqlite3_parser::{batch, incr, slot};

    use pretty_assertions::assert_eq;

    /// Autocomplete position is indicated by `. If the position is at the end of the sql string
    /// ` is not needed.
    fn autocomplete_sql<CST: CstTrait>(sql_with_cursor: &str) -> Vec<String> {
        let autocomplete_cursor = TextSize::new(
            sql_with_cursor
                .chars()
                .position(|it| it == '`')
                .unwrap_or(sql_with_cursor.chars().count()) as u32,
        );

        let sql = sql_with_cursor.replace("`", "");
        let ast: CST = bord_sqlite3_parser::parse(&sql);

        create_completion_context(&ast, autocomplete_cursor)
    }

    macro_rules! testcase {
        ($testname:ident, $sql:expr, $expected:expr) => {
            #[test]
            fn $testname() {
                let completions = autocomplete_sql::<incr::IncrSqlCst>($sql);

                let completions_set = completions.into_iter().sorted().collect_vec();
                let expected = $expected
                    .iter()
                    .map(|it| it.to_string())
                    .sorted()
                    .collect_vec();

                assert_eq!(completions_set, expected);

                let completions = autocomplete_sql::<batch::SqlCst>($sql);

                let completions_set = completions.into_iter().sorted().collect_vec();
                let expected = $expected
                    .iter()
                    .map(|it| it.to_string())
                    .sorted()
                    .collect_vec();

                assert_eq!(completions_set, expected);

                let completions = autocomplete_sql::<slot::SlotIncrSqlCst>($sql);

                let completions_set = completions.into_iter().sorted().collect_vec();
                let expected = $expected
                    .iter()
                    .map(|it| it.to_string())
                    .sorted()
                    .collect_vec();

                assert_eq!(completions_set, expected);
            }
        };
    }

    testcase!(test_autocomplete1, "SELECT", &["DISTINCT", "ALL"]);

    testcase!(
        test_autocomplete2,
        "SELECT *",
        &[
            "LIMIT",
            "HAVING",
            "INTERSECT",
            "EXCEPT",
            "FROM",
            "WINDOW",
            "UNION",
            "GROUP BY",
            "ORDER BY",
            "WHERE"
        ]
    );

    testcase!(
        test_autocomplete3,
        "SELECT * FROM users ` GROUP BY name",
        &[
            "AS",
            "LIMIT",
            "HAVING",
            "INTERSECT",
            "EXCEPT",
            "WINDOW",
            "INDEXED BY",
            "NOT INDEXED",
            "UNION",
            "GROUP BY",
            "ORDER BY",
            "WHERE"
        ]
    );

    testcase!(
        test_autocomplete4,
        r#"SELECT x, y, row_number() OVER win1, rank() OVER win2
        FROM t0
        WINDOW win1 AS (ORDER BY y RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW),
        win2 AS (PARTITION BY y ORDER BY x) "#,
        &["EXCEPT", "INTERSECT", "LIMIT", "ORDER BY", "UNION"]
    );

    testcase!(
        test_autocomplete5,
        "EXPLAIN",
        &[
            "ALTER TABLE",
            "ANALYZE",
            "ATTACH",
            "BEGIN",
            "COMMIT",
            "CREATE INDEX",
            "CREATE TABLE",
            "CREATE TRIGGER",
            "CREATE",
            "CREATE VIEW",
            "CREATE VIRTUAL TABLE",
            "DELETE FROM",
            "DETACH",
            "DROP INDEX",
            "DROP TABLE",
            "DROP TRIGGER",
            "DROP VIEW",
            "END",
            "INSERT",
            "INSERT INTO",
            "PRAGMA",
            "QUERY PLAN",
            "REINDEX",
            "RELEASE",
            "REPLACE INTO",
            "ROLLBACK",
            "SAVEPOINT",
            "SELECT",
            "UPDATE",
            "VACUUM",
            "VALUES",
            "WITH"
        ]
    );

    testcase!(test_autocomplete6, "EXPLAIN ATTACH", &["DATABASE"]);

    testcase!(
        test_autocomplete7,
        "ALTER TABLE users ",
        &["ADD", "DROP", "RENAME", "RENAME TO"]
    );

    testcase!(
        test_autocomplete8,
        "INSERT OR IGNORE INTO users VALUES (1, 2)",
        &["ON CONFLICT", "RETURNING"]
    );

    testcase!(
        test_autocomplete9,
        "INSERT OR IGNORE INTO users VALUES (1, 2) ON CONFLICT",
        &["DO NOTHING", "DO UPDATE SET"]
    );

    testcase!(
        test_autocomplete10,
        "INSERT OR IGNORE INTO users VALUES (1, 2) ON CONFLICT DO",
        &["NOTHING", "UPDATE SET"]
    );
}
