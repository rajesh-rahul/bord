use crate::CstNodeTrait;
use std::collections::HashMap;
use std::sync::LazyLock;
pub use ungrammar::{Node, NodeData, Rule, Token, TokenData};

pub static UNGRAMMAR: LazyLock<Ungrammar> = LazyLock::new(|| Ungrammar::new());

pub struct Ungrammar {
    inner: ungrammar::Grammar,
    map: HashMap<String, ungrammar::Node>,
    root: Rule,
}

impl Ungrammar {
    pub fn new() -> Self {
        let input = include_str!("../../sqlite.ungram");
        let ungram = input.parse::<ungrammar::Grammar>().unwrap();

        let map: HashMap<_, _> = ungram
            .iter()
            .map(|it| (ungram[it].name.clone(), it))
            .collect();

        let root_id = *map.get("File").unwrap();

        Self {
            inner: ungram,
            map,
            root: Rule::Node(root_id),
        }
    }

    pub fn get_by_name(&self, name: &str) -> Option<&ungrammar::NodeData> {
        self.map.get(name).map(|it| &self.inner[*it])
    }

    pub fn root(&self) -> &Rule {
        &self.root
    }

    pub fn get_node(&self, node_id: Node) -> &NodeData {
        &self.inner[node_id]
    }

    pub fn nodes(&self) -> impl Iterator<Item = &NodeData> {
        self.inner.iter().map(|it| &self.inner[it])
    }

    pub fn tokens(&self) -> impl Iterator<Item = &TokenData> {
        self.inner.tokens().map(|it| &self.inner[it])
    }
    pub fn get_token(&self, token_id: Token) -> &str {
        &self.inner[token_id].name
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UngramNode<T> {
    Rule(&'static ungrammar::Rule),
    RepMarker {
        rule: &'static ungrammar::Rule,
        parent_list_len: usize,
        ast_stack_idx: usize,
        ast_stack_item: T,
    },
    AltMarker {
        alt_marker_idx: usize,
        alt_marker_end_idx: usize,
        parent_list_len: usize,
        ast_stack_idx: usize,
        ast_stack_item: T,
    },
    OptMarker {
        parent_list_len: usize,
        ast_stack_idx: usize,
        ast_stack_item: T,
    },
    AltItemBegin,
}

struct FallbackLocation<T> {
    ungram_stack_len: usize,
    ast_stack_idx: usize,
    ast_stack_item: T,
    node_list_len: usize,
}

pub struct UngramTraverser<T> {
    ast_stack: Vec<T>,
    ungram_stack: Vec<UngramNode<T>>,
    rules_history: Vec<&'static Rule>,
    // ast: &'a SqliteUntypedCst,
}

pub enum UngramTraverserNodeKind<T> {
    Token {
        name: &'static str,
        ast_node: Option<T>,
        rule: &'static Rule,
    },
    Tree {
        name: &'static str,
        rule: &'static Rule,
        ast_node: Option<T>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum UngramTraverserBacktrackResult {
    Success,
    Fail,
}

impl<'a, T: Copy + CstNodeTrait<'a>> UngramTraverser<T> {
    pub fn new(ast_root: T, ungram_root: &'static Rule) -> Self {
        let ast_stack = vec![ast_root];
        let ungram_stack = vec![UngramNode::Rule(ungram_root)];
        Self {
            ast_stack,
            ungram_stack,
            rules_history: Vec::new(),
        }
    }

    pub fn rules_history(&self) -> &[&'static Rule] {
        &self.rules_history
    }

    pub fn ungram_stack(&self) -> &[UngramNode<T>] {
        &self.ungram_stack
    }

    pub fn ast_stack(&self) -> &[T] {
        &self.ast_stack
    }

    pub fn next(&mut self) -> Option<UngramTraverserNodeKind<T>> {
        while !self.ast_stack.is_empty() || !self.ungram_stack.is_empty() {
            let ungram_node = self.ungram_stack.last();

            let ungram_rule = match ungram_node {
                Some(UngramNode::RepMarker { rule, .. }) => {
                    if self.ast_stack.is_empty() {
                        self.ungram_stack.pop();
                        continue;
                    } else {
                        let updated_marker = UngramNode::RepMarker {
                            rule,
                            parent_list_len: self.rules_history.len(),
                            ast_stack_idx: self.ast_stack.len() - 1,
                            ast_stack_item: *self
                                .ast_stack
                                .last()
                                .expect("DEV ERROR: self.ast_stack len guaranteed > 0"),
                        };
                        let rule_copied = UngramNode::Rule(rule);
                        let ungram_node_idx = self.ungram_stack.len() - 1; // Guaranteed to exist
                        self.ungram_stack[ungram_node_idx] = updated_marker;
                        self.ungram_stack.push(rule_copied);
                    }
                    continue;
                }
                Some(UngramNode::AltMarker { .. }) => {
                    self.ungram_stack.pop().expect("DEV ERROR: guaranteed pop");
                    continue;
                }
                Some(UngramNode::OptMarker { .. }) => {
                    self.ungram_stack.pop().expect("DEV ERROR: guaranteed pop");
                    continue;
                }
                Some(UngramNode::AltItemBegin) => {
                    assert!(self.rules_history.len() > 0);
                    let rule_to_keep = *self.rules_history.last().unwrap();
                    while !matches!(self.ungram_stack.last(), Some(UngramNode::AltMarker { .. })) {
                        if matches!(self.ungram_stack.last(), Some(UngramNode::AltItemBegin)) {
                            // eprintln!("ungram stack: {:?}", self.ungram_stack);
                            // let poped = self.node_list.pop().unwrap();
                            // eprintln!("popped from alt{:?}", poped);
                        }

                        self.ungram_stack.pop().expect(
                            "DEV ERROR: Expected items in stack if AltMarker is not yet seen",
                        );
                    }

                    self.rules_history.push(rule_to_keep);

                    self.ungram_stack
                        .pop()
                        .expect("DEV ERROR: Expected to pop AltMarker");
                    continue;
                }
                None => break,
                Some(UngramNode::Rule(rule)) => {
                    self.rules_history.push(rule);

                    rule
                }
            };

            match &ungram_rule {
                Rule::Labeled { rule, .. } => {
                    self.ungram_stack.pop().unwrap();
                    self.ungram_stack.push(UngramNode::Rule(rule));
                }
                Rule::Opt(rule) => {
                    self.ungram_stack.pop().unwrap();
                    if self.ast_stack.is_empty() {
                        continue;
                    }

                    let ast_stack_item = *self
                        .ast_stack
                        .last()
                        .expect("DEV ERROR: Atleast one element expected");

                    let marker = UngramNode::OptMarker {
                        parent_list_len: self.rules_history.len(),
                        ast_stack_idx: self.ast_stack.len() - 1,
                        ast_stack_item,
                    };
                    self.ungram_stack.push(marker);
                    self.ungram_stack.push(UngramNode::Rule(rule));
                }
                Rule::Rep(rule) => {
                    self.ungram_stack.pop().unwrap();
                    if self.ast_stack.is_empty() {
                        continue;
                    }

                    let ast_stack_item = *self
                        .ast_stack
                        .last()
                        .expect("DEV ERROR: Atleast one element expected");

                    let marker = UngramNode::RepMarker {
                        parent_list_len: self.rules_history.len(),
                        rule,
                        ast_stack_idx: self.ast_stack.len() - 1,
                        ast_stack_item,
                    };
                    self.ungram_stack.push(marker);
                    self.ungram_stack.push(UngramNode::Rule(rule));
                }
                Rule::Seq(vec) => {
                    self.ungram_stack.pop().unwrap();
                    for rule in vec.iter().rev() {
                        self.ungram_stack.push(UngramNode::Rule(rule));
                    }
                }
                Rule::Alt(vec) => {
                    self.ungram_stack.pop().unwrap();
                    if self.ast_stack.is_empty() {
                        break;
                    }

                    let ast_stack_item = *self
                        .ast_stack
                        .last()
                        .expect("DEV ERROR: Atleast one element expected");

                    let mut alt_marker_end_idx = self.ungram_stack.len();
                    let alt_marker_idx = self.ungram_stack.len();
                    self.ungram_stack.push(UngramNode::AltItemBegin); // Dummy value, we change it at the end

                    for rule in vec.iter().rev() {
                        alt_marker_end_idx += 2;
                        self.ungram_stack.push(UngramNode::AltItemBegin);
                        self.ungram_stack.push(UngramNode::Rule(rule));
                    }

                    let marker = UngramNode::AltMarker {
                        alt_marker_idx,
                        alt_marker_end_idx,
                        parent_list_len: self.rules_history.len(),
                        ast_stack_idx: self.ast_stack.len() - 1,
                        ast_stack_item,
                    };

                    self.ungram_stack[alt_marker_idx] = marker; // Update dummy value to real value
                }
                Rule::Node(node_id) => {
                    return Some(UngramTraverserNodeKind::Tree {
                        name: &UNGRAMMAR.get_node(*node_id).name,
                        rule: ungram_rule,
                        ast_node: self.ast_stack.last().map(|it| *it),
                    });
                }
                Rule::Token(token_id) => {
                    return Some(UngramTraverserNodeKind::Token {
                        name: UNGRAMMAR.get_token(*token_id),
                        rule: ungram_rule,
                        ast_node: self.ast_stack.last().map(|it| *it),
                    })
                }
            }
        }

        return None;
    }

    pub fn backtrack(&mut self) -> UngramTraverserBacktrackResult {
        if let Some(marker) = self.find_fallback_marker() {
            while self.ungram_stack.len() > marker.ungram_stack_len {
                self.ungram_stack
                    .pop()
                    .expect("DEV ERROR: ungram_stack len is always > 0");
            }
            while self.rules_history.len() >= marker.node_list_len {
                self.rules_history
                    .pop()
                    .expect("DEV ERROR: ungram_stack len is always > 0");
            }
            while self.ast_stack.len() > marker.ast_stack_idx {
                self.ast_stack
                    .pop()
                    .expect("DEV ERROR: ungram_stack len is always > 0");
            }

            if !self.ast_stack.is_empty() || !self.ungram_stack.is_empty() {
                self.ast_stack.push(marker.ast_stack_item);
            }
            UngramTraverserBacktrackResult::Success
        } else {
            UngramTraverserBacktrackResult::Fail
        }
    }

    fn find_fallback_marker(&mut self) -> Option<FallbackLocation<T>> {
        for (idx, node) in self.ungram_stack.iter_mut().enumerate().rev() {
            match node {
                UngramNode::RepMarker {
                    ast_stack_idx,
                    ast_stack_item,
                    parent_list_len,
                    ..
                } => {
                    return Some(FallbackLocation {
                        ungram_stack_len: idx,
                        ast_stack_idx: *ast_stack_idx,
                        ast_stack_item: *ast_stack_item,
                        node_list_len: *parent_list_len,
                    })
                }
                UngramNode::AltMarker {
                    alt_marker_idx,
                    alt_marker_end_idx,
                    ast_stack_idx,
                    ast_stack_item,
                    parent_list_len,
                } => {
                    *alt_marker_end_idx -= 2;
                    // *parent_list_len -= 1;

                    if alt_marker_end_idx > alt_marker_idx {
                        return Some(FallbackLocation {
                            ungram_stack_len: *alt_marker_end_idx + 1,
                            ast_stack_idx: *ast_stack_idx,
                            ast_stack_item: *ast_stack_item,
                            node_list_len: *parent_list_len + 1,
                        });
                    } else {
                        continue;
                    }
                }
                UngramNode::OptMarker {
                    ast_stack_idx,
                    ast_stack_item,
                    parent_list_len,
                } => {
                    return Some(FallbackLocation {
                        ungram_stack_len: idx,
                        ast_stack_idx: *ast_stack_idx,
                        ast_stack_item: *ast_stack_item,
                        node_list_len: *parent_list_len,
                    })
                }
                _ => continue,
            }
        }

        return None;
    }

    pub fn token_visited(&mut self) {
        // TODO: Why this match statement for tokens? (Copy paste error?)
        let ungram_rule = match self.ungram_stack.pop() {
            Some(UngramNode::Rule(rule)) => match rule {
                Rule::Node(node_id) => &UNGRAMMAR.get_node(*node_id).rule,
                _ => rule,
            },
            _ => panic!("DEV ERROR: API Misuse"),
        };

        if !matches!(&ungram_rule, Rule::Token(_)) {
            panic!("DEV ERROR: API Misuse")
        }

        if !self.ast_stack.pop().and_then(|it| it.token()).is_some() {
            panic!("DEV ERROR: API Misuse")
        }
    }

    pub fn ignore_token_because_err(&mut self) {
        // TODO: Why this match statement for tokens? (Copy paste error?)
        let ungram_rule = match self.ungram_stack.pop() {
            Some(UngramNode::Rule(rule)) => match rule {
                Rule::Node(node_id) => &UNGRAMMAR.get_node(*node_id).rule,
                _ => rule,
            },
            _ => panic!("DEV ERROR: API Misuse"),
        };

        if !matches!(&ungram_rule, Rule::Token(_)) {
            panic!("DEV ERROR: API Misuse")
        }

        if !self.ast_stack.pop().and_then(|it| it.error()).is_some() {
            panic!("DEV ERROR: API Misuse")
        }
    }

    pub fn ignore_node_because_err(&mut self) {
        if !matches!(
            &self.ungram_stack.pop(),
            Some(UngramNode::Rule(Rule::Node(_)))
        ) {
            panic!("DEV ERROR: API Misuse")
        }

        if !self.ast_stack.pop().and_then(|it| it.error()).is_some() {
            panic!("DEV ERROR: API Misuse")
        }
    }

    pub fn node_visited_and_expand_children(&mut self) {
        let ungram_rule = match self.ungram_stack.pop() {
            Some(UngramNode::Rule(rule)) => match rule {
                Rule::Node(node_id) => &UNGRAMMAR.get_node(*node_id).rule,
                _ => rule,
            },
            _ => panic!("DEV ERROR: API Misuse"),
        };

        self.ungram_stack.push(UngramNode::Rule(ungram_rule));

        match self.ast_stack.pop() {
            Some(node) if node.tree().is_some() => {
                for child in node.non_trivial_children().rev() {
                    self.ast_stack.push(child);
                }
            }
            _ => panic!("DEV ERROR: API Misuse"),
        }
    }

    pub fn node_visited(&mut self) {
        if !matches!(
            &self.ungram_stack.pop(),
            Some(UngramNode::Rule(Rule::Node(_)))
        ) {
            panic!("DEV ERROR: API Misuse")
        }

        if !matches!(self.ast_stack.pop(), Some(node) if node.tree().is_some()) {
            panic!("DEV ERROR: API Misuse")
        }
    }

    pub fn is_traversal_complete(&self) -> bool {
        self.ast_stack.is_empty() && self.ungram_stack.is_empty()
    }
}

pub fn rule_to_str(r: &Rule) -> String {
    match r {
        Rule::Labeled { label, rule } => format!("{label} : {}", rule_to_str(rule)),
        Rule::Node(node) => format!("{}", UNGRAMMAR.get_node(*node).name),
        Rule::Token(token) => format!("{}", UNGRAMMAR.get_token(*token)),
        Rule::Seq(vec) => format!(
            "[{}]",
            vec.iter().map(rule_to_str).collect::<Vec<_>>().join(", ")
        ),
        Rule::Alt(vec) => vec.iter().map(rule_to_str).collect::<Vec<_>>().join(" | "),
        Rule::Opt(rule) => format!("({})?", rule_to_str(rule)),
        Rule::Rep(rule) => format!("({})*", rule_to_str(rule)),
    }
}
