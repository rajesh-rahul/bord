use bord_sqlite3_parser::{sqlite_keywords, ungram::UNGRAMMAR};
use quote::{format_ident, quote};

#[test]
fn test_ungram_keywords_are_correct() {
    for token_data in UNGRAMMAR.tokens() {
        if token_data.name.starts_with("KW_") {
            let keyword = token_data.name.trim_start_matches("KW_");
            if sqlite_keywords(keyword.as_bytes()).is_none() {
                panic!("{} is not a keyword", token_data.name)
            }
        } else {
            if sqlite_keywords(token_data.name.as_bytes()).is_some() {
                panic!("{} is a keyword", token_data.name)
            }
        }
    }
}

#[test]
fn test_tree_kinds_are_correct() {
    let input = include_str!("../../sqlite3-parser/src/tree_kind.rs");

    let enum_variants = UNGRAMMAR
        .nodes()
        .map(|node| format_ident!("{}", &node.name));

    let actual = syn::parse_file(input).unwrap();
    let actual_tree_kind = actual
        .items
        .iter()
        .find(|it| matches!(it, syn::Item::Enum(_)))
        .unwrap();

    let expected = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub enum SqliteTreeKind {
            #(#enum_variants ,)*
        }
    };

    let expected_file = syn::parse_file(&expected.to_string()).unwrap();
    let expected_tree_kind = expected_file
        .items
        .iter()
        .find(|it| matches!(it, syn::Item::Enum(_)))
        .unwrap();

    println!(
        "{}",
        prettyplease::unparse(&syn::parse_file(&expected.to_string()).unwrap())
    );
    assert!(actual_tree_kind == expected_tree_kind);
}
