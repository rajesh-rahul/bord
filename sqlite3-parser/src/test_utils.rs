use crate::grammar::common::{IDEN_SET, JOIN_KEYWORDS};
use crate::ungram::{
    UngramTraverser, UngramTraverserBacktrackResult, UngramTraverserNodeKind, UNGRAMMAR,
};
use crate::{
    CstNode, CstNodeData, SqliteLexer, SqliteParseError, SqliteParser, SqliteTokenKind,
    SqliteUntypedCst, SqliteVersion,
};
use core::str;
use nom::bytes::complete::escaped;
use nom::character::complete::{multispace0, none_of};
use nom::{
    bytes::complete::tag, character::complete::alphanumeric1 as alphanumeric, error::VerboseError,
    sequence::delimited, IResult,
};
use pretty_assertions::assert_eq;

pub fn ensure_ast_conforms_to_ungram(ast: &SqliteUntypedCst) {
    let mut traverser = UngramTraverser::new(ast.root(), &UNGRAMMAR.root());

    let mut before_backtrack = None;
    let mut match_history = Vec::new();

    while let Some(result) = traverser.next() {
        match result {
            UngramTraverserNodeKind::Token { name, ast_node, .. } => match ast_node {
                Some(
                    node @ CstNode {
                        data: CstNodeData::Token(tk),
                        ..
                    },
                ) if node.as_str() == name.trim_start_matches("KW_")
                    || fat_ungram_tokens_match_ast(name, tk.kind) =>
                {
                    traverser.token_visited();
                    before_backtrack = None;
                    match_history.push(name.to_owned());
                }
                Some(
                    node @ CstNode {
                        data: CstNodeData::Error(err),
                        ..
                    },
                ) if parse_err_corresponds_to_ungram_item(name, node.error().unwrap()) => {
                    traverser.ignore_token_because_err();
                    before_backtrack = None;
                    match_history.push(format!("Ignored: {name}"));
                }
                _ => {
                    let ast_node_as_string = ast_node
                        .map(|it| it.as_str().to_owned())
                        .unwrap_or("None".to_string());
                    if before_backtrack.is_none() {
                        before_backtrack = Some((name.to_owned(), ast_node_as_string));
                    }

                    if traverser.backtrack() == UngramTraverserBacktrackResult::Fail {
                        if let Some((ungrammar_expected, ast_value)) = before_backtrack {
                            panic!("Ungrammar expected token {ungrammar_expected} but ast had {ast_value}\nMatch History: {match_history:?}")
                        } else {
                            panic!("Ungrammar mismatch")
                        }
                    }
                }
            },
            UngramTraverserNodeKind::Tree { name, ast_node, .. } => match ast_node {
                Some(
                    node @ CstNode {
                        data: CstNodeData::Tree(_),
                        ..
                    },
                ) if node.as_str() == name => {
                    traverser.node_visited_and_expand_children();
                    before_backtrack = None;
                    match_history.push(name.to_owned());
                }
                Some(
                    node @ CstNode {
                        data: CstNodeData::Error(err),
                        ..
                    },
                ) if parse_err_corresponds_to_ungram_item(name, node.error().unwrap()) => {
                    traverser.ignore_node_because_err();
                    match_history.push(format!("Ignored: {name}"));
                    before_backtrack = None;
                }
                _ => {
                    let ast_node_as_string = ast_node
                        .map(|it| it.as_str().to_owned())
                        .unwrap_or("None".to_string());
                    if before_backtrack.is_none() {
                        before_backtrack = Some((name.to_owned(), ast_node_as_string));
                    }
                    if traverser.backtrack() == UngramTraverserBacktrackResult::Fail {
                        if let Some((ungrammar_expected, ast_value)) = before_backtrack {
                            panic!("Ungrammar expected {ungrammar_expected} but ast had {ast_value}\nMatch History: {match_history:?}")
                        } else {
                            panic!("Ungrammar mismatch")
                        }
                    }
                }
            },
        }
    }

    if !traverser.is_traversal_complete() {
        panic!(
            "Ungrammar and Ast mismatch!\nHistory: {match_history:?}\nAst: {:?}\nUngrammar: {:?}",
            traverser.ast_stack(),
            traverser.ungram_stack()
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TestInput {
    pub input: String,
    pub expected: Vec<SimpleSqliteNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SimpleSqliteNode {
    SqliteNode {
        name: String,
        children: Vec<SimpleSqliteNode>,
    },
    Leaf(String),
}

pub fn check_ast(
    parse_function: fn(&mut SqliteParser, r: enumset::EnumSet<SqliteTokenKind>),
    r: enumset::EnumSet<SqliteTokenKind>,
    input: &str,
    expected_ast: &str,
) {
    let ast = crate::parse_any(input, r, parse_function);
    check_input(&ast, expected_ast)
}

pub fn parse_is_at(func: fn(&SqliteParser) -> bool, input: &str) -> bool {
    let (tokens, _) = SqliteLexer::new(input, SqliteVersion([3, 46, 0])).lex();

    let p = SqliteParser::new(tokens);

    func(&p)
}

pub fn check_input(actual_tree: &SqliteUntypedCst, expected_as_str: &str) {
    // let (_, data) = test_data(&test_input).unwrap();
    let actual = convert_tree_to_simple_node(&actual_tree);
    let (_, expected) = str_to_simple_node(expected_as_str).unwrap();

    println!("{actual_tree}");
    if actual != expected {
        let mut actual_str = String::new();
        print_simple_node(&actual, 0, &mut actual_str, true);

        let mut expected_str = String::new();
        print_simple_node(&expected, 0, &mut expected_str, true);

        let mut actual_str_wide = String::new();
        print_simple_node(&actual, 0, &mut actual_str_wide, false);

        println!("Actual str:\n{}", actual_str_wide);
        assert_eq!(expected_str, actual_str);
    }
}

fn parse_err_corresponds_to_ungram_item(ungram_item: &str, err: &SqliteParseError) -> bool {
    use crate::parser::{ExpectedItem, ParseErrorKind};

    match &err.message {
        ParseErrorKind::ExpectedItem(vec) => vec.iter().any(|it| match it {
            ExpectedItem::Token(tk_kind) => {
                tk_kind.as_str() == ungram_item.trim_start_matches("KW_")
                    || fat_ungram_tokens_match_ast(ungram_item, *tk_kind)
            }
            ExpectedItem::Tree(tree_kind) => ungram_item == Into::<&str>::into(tree_kind),
        }),
        ParseErrorKind::UnknownTokens => false,
        ParseErrorKind::OtherError(_) => false,
    }
}

fn fat_ungram_tokens_match_ast(ungram_token: &str, ast_token: SqliteTokenKind) -> bool {
    use SqliteTokenKind::*;

    if !ungram_token.starts_with("$") {
        return false;
    }

    assert!(matches!(
        ungram_token,
        "$NAME" | "$CONSTRAINT_NAME" | "$NUMERIC_LIT" | "$LITERAL_VALUE" | "$COLLATION_NAME"
    ));

    // NOTE: In real scenarios actual iden set is dynamic However, in testing scenarios
    // such as this, we will default use the static IDEN_SET
    let name_static_set = IDEN_SET | KW_INDEXED | JOIN_KEYWORDS;
    match (ungram_token, ast_token) {
        ("$NAME" | "$CONSTRAINT_NAME", ast_token) if (name_static_set).contains(ast_token) => true,
        ("$NUMERIC_LIT", INT_LIT | HEX_LIT | REAL_LIT) => true,
        ("$COLLATION_NAME", ast_token) if (IDEN_SET | STR_LIT).contains(ast_token) => true,
        (
            "$LITERAL_VALUE",
            INT_LIT | HEX_LIT | REAL_LIT | STR_LIT | BLOB_LIT | KW_NULL | KW_TRUE | KW_FALSE
            | KW_CURRENT_TIME | KW_CURRENT_DATE,
        ) => true,
        _ => false,
    }
}

// leaf looks like this "SELECT"
fn str_to_simple_leaf(i: &str) -> IResult<&str, SimpleSqliteNode, VerboseError<&str>> {
    let (i, _) = tag("\"")(i)?;
    // let (i, leaf) = delimited(tag("\""), escaped(take_until("\""), '\\', one_of(r#""n\"#)), tag("\""))(i)?;
    let (i, leaf) = escaped(none_of("\"\\"), '\\', tag("\""))(i)?;
    let (i, _) = tag("\"")(i)?;

    Ok((
        i,
        SimpleSqliteNode::Leaf(leaf.to_string().replace("\\", "")),
    ))
}

fn str_to_simple_node<'a>(i: &'a str) -> IResult<&'a str, SimpleSqliteNode, VerboseError<&'a str>> {
    let (i, name) = delimited(multispace0, alphanumeric, multispace0)(i)?;
    let (mut input, _) = delimited(multispace0, tag("{"), multispace0)(i)?;

    let mut children = Vec::new();

    loop {
        if let Ok((i, _)) = delimited(
            multispace0::<_, VerboseError<&'a str>>,
            tag("}"),
            multispace0,
        )(input)
        {
            input = i;
            break;
        }

        if let Ok((i, child)) = delimited(multispace0, str_to_simple_leaf, multispace0)(input) {
            input = i;
            children.push(child);
            continue;
        }

        if let Ok((i, child)) = delimited(multispace0, str_to_simple_node, multispace0)(input) {
            input = i;
            children.push(child);
            continue;
        }
    }

    Ok((
        input,
        SimpleSqliteNode::SqliteNode {
            name: name.to_string(),
            children,
        },
    ))
}

fn into_simple_node(input: CstNode, ast: &SqliteUntypedCst) -> Option<SimpleSqliteNode> {
    let simple_node = match input.data {
        CstNodeData::Tree(tree_kind) => SimpleSqliteNode::SqliteNode {
            name: format!("{:?}", tree_kind),
            children: input
                .children()
                .flat_map(|child| into_simple_node(child, ast))
                .collect(),
        },
        CstNodeData::Token(token) if !token.is_trivia() => match token.kind {
            SqliteTokenKind::IDEN
            | SqliteTokenKind::HEX_LIT
            | SqliteTokenKind::BLOB_LIT
            | SqliteTokenKind::INT_LIT
            | SqliteTokenKind::STR_LIT
            | SqliteTokenKind::REAL_LIT => SimpleSqliteNode::Leaf(format!(
                "{text} - {str_rep}",
                str_rep = token.kind.as_str().to_owned(),
                text = token.text
            )),

            _ => SimpleSqliteNode::Leaf(token.kind.as_str().to_owned()),
        },
        CstNodeData::Error(_) => SimpleSqliteNode::SqliteNode {
            name: format!("Error"),
            children: input
                .children()
                .flat_map(|child| into_simple_node(child, ast))
                .collect(),
        },
        _ => return None,
    };

    Some(simple_node)
}

fn convert_tree_to_simple_node(ast: &SqliteUntypedCst) -> SimpleSqliteNode {
    let root = ast.root();

    // If the root is the File node, hide it so our test data is a bit cleaner
    match root {
        // SqliteNode::Tree(
        //     tree @ TreeChild {
        //         kind: SqliteTreeKind::File,
        //         ..
        //     },
        // ) if tree
        //     .children(ast)
        //     .filter_map(|child| child.tree_kind())
        //     .count()
        //     == 1 =>
        // {
        //     tree.children(ast)
        //         .filter_map(|stmt| match stmt {
        //             stmt @ SqliteNode::Tree { .. } => into_simple_node(&stmt, ast),
        //             _ => None,
        //         })
        //         .next()
        //         .unwrap()
        // }
        _ => into_simple_node(root, ast).unwrap(),
    }
}

fn print_simple_node(node: &SimpleSqliteNode, indent: usize, write: &mut String, compact: bool) {
    let indent_str = " ".repeat(indent);

    match node {
        SimpleSqliteNode::SqliteNode { name, children } => {
            if compact {
                write.push_str(&format!("{}{}\n", indent_str, name));
            } else {
                write.push_str(&format!("{}{} {{\n", indent_str, name));
            }

            for child in children {
                print_simple_node(child, indent + 4, write, compact)
            }

            if !compact {
                write.push_str(&format!("{}}}\n", indent_str));
            }
        }
        SimpleSqliteNode::Leaf(leaf) => {
            write.push_str(&format!("{}\"{}\"\n", " ".repeat(indent), leaf));
        }
    }
}
