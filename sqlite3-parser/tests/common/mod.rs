use nom::bytes::complete::escaped;
use nom::character::complete::{multispace0, none_of};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::alphanumeric1 as alphanumeric,
    error::VerboseError,
    sequence::delimited,
    IResult,
};
use yukon_sqlite3_parser::{SqliteNode, SqliteUntypedAst, SqliteTreeKind};

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

fn input_sql<'a>(i: &'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
    let i = i.trim_start(); // Easy way to remove leading whitespace
    let (i, input_sql) = take_until("\n\n")(i)?;
    let (i, _) = tag("\n\n")(i)?;

    Ok((i, input_sql))
}

// leaf looks like this "SELECT"
fn simple_leaf(i: &str) -> IResult<&str, SimpleSqliteNode, VerboseError<&str>> {
    let (i, _) = tag("\"")(i)?;
    // let (i, leaf) = delimited(tag("\""), escaped(take_until("\""), '\\', one_of(r#""n\"#)), tag("\""))(i)?;
    let (i, leaf) = escaped(none_of("\"\\"), '\\', tag("\""))(i)?;
    let (i, _) = tag("\"")(i)?;

    Ok((i, SimpleSqliteNode::Leaf(leaf.to_string().replace("\\", ""))))
}

fn simple_node<'a>(i: &'a str) -> IResult<&'a str, SimpleSqliteNode, VerboseError<&'a str>> {
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

        if let Ok((i, child)) = delimited(multispace0, simple_leaf, multispace0)(input) {
            input = i;
            children.push(child);
            continue;
        }

        if let Ok((i, child)) = delimited(multispace0, simple_node, multispace0)(input) {
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

pub fn test_data<'a>(i: &'a str) -> IResult<&'a str, TestInput, VerboseError<&'a str>> {
    let (mut i, input_sql) = input_sql(i)?;

    let mut expected = Vec::new();
    while let Ok((rest, node)) = simple_node(i) {
        i = rest;
        expected.push(node);

        if let Ok((rest, _)) =
            delimited(multispace0::<_, VerboseError<&str>>, tag("%%"), multispace0)(i)
        {
            i = rest;
            break;
        }
    }

    Ok((
        i,
        TestInput {
            input: input_sql.to_string(),
            expected,
        },
    ))
}

fn into_simple_node(input: &SqliteNode, ast: &SqliteUntypedAst) -> Option<SimpleSqliteNode> {
    let simple_node = match input {
        SqliteNode::Tree { kind, .. } => SimpleSqliteNode::SqliteNode {
            name: format!("{:?}", kind),
            children: input
                .children(ast)
                .flat_map(|child| into_simple_node(child, ast))
                .collect(),
        },
        SqliteNode::Token { token, .. } if !token.is_trivia() => {
            SimpleSqliteNode::Leaf(token.as_str().to_owned())
        }
        _ => return None,
    };

    Some(simple_node)
}

pub fn convert_tree_to_simple_node(ast: &SqliteUntypedAst) -> Vec<SimpleSqliteNode> {
    let root = ast.root();
    assert!(matches!(root.tree_kind(), Some(SqliteTreeKind::File)));

    root.children(ast)
        .filter_map(|stmt| match stmt {
            stmt @ SqliteNode::Tree { .. } => into_simple_node(stmt, ast),
            _ => None,
        })
        .collect()
}

pub fn print_simple_node(node: &SimpleSqliteNode, indent: usize, write: &mut String, compact: bool) {
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
