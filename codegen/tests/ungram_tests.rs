use quote::{format_ident, quote};
use std::sync::LazyLock;
use bord_sqlite3_parser::sqlite_keywords;

static UNGRAMMAR: LazyLock<ungrammar::Grammar> = LazyLock::new(|| {
    let input = include_str!("../../sqlite.ungram");

    input.parse::<ungrammar::Grammar>().unwrap()
});

#[test]
fn test_ungram_keywords_are_correct() {
    for token in UNGRAMMAR.tokens() {
        let token_data = &UNGRAMMAR[token];

        if token_data.name.starts_with("KW_") {
            let keyword = token_data.name.trim_start_matches("KW_");
            if sqlite_keywords().get(keyword.as_bytes()).is_none() {
                panic!("{} is not a keyword", token_data.name)
            }
        } else {
            if sqlite_keywords().get(token_data.name.as_bytes()).is_some() {
                panic!("{} is a keyword", token_data.name)
            }
        }
    }
}

#[test]
fn test_tree_kinds_are_correct() {
    let mut s = String::new();

    s.push_str("pub");

    let input = include_str!("../../sqlite3-parser/src/tree_kind.rs");

    let enum_variants = UNGRAMMAR
        .iter()
        .map(|node| format_ident!("{}", &UNGRAMMAR[node].name));

    let actual = syn::parse_file(input).unwrap();

    let expected = quote! {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, strum::IntoStaticStr)]
        pub enum SqliteTreeKind {
            #(#enum_variants ,)*
        }
    };

    assert!(actual == syn::parse_file(&expected.to_string()).unwrap());
}
