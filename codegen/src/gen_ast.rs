use std::collections::{HashMap, HashSet};

use bord_sqlite3_parser::ungram::{rule_to_str, Rule, UNGRAMMAR};
use convert_case::{Case, Casing};
use itertools::Itertools;
use quote::{format_ident, quote};

#[derive(Debug)]
struct RustStruct {
    name: String,
    methods: Vec<RustMethod>,
}

#[derive(Debug)]
struct RustMethod {
    name: String,
    search_kind: SearchKind,
    return_ty: String,
}

#[derive(Debug)]
struct RustEnum {
    name: String,
    fields: Vec<RustEnumField>,
    is_non_terminal_node: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct RustEnumField {
    name: String,
    kind: RustEnumFieldKind,
}

#[derive(Debug, PartialEq, Eq)]
enum RustEnumFieldKind {
    Node,
    Token,
}

#[derive(Debug, PartialEq, Eq)]
enum SearchKind {
    Node,
    TokenSet,
    Token,
    Rep,
    Alt,
}

#[derive(Default)]
struct AstGenerator {
    enums: HashMap<String, RustEnum>,
    structs: HashMap<String, RustStruct>,
}

fn main() {
    // Ensure all nodes have either Alt or Seq as child
    let mut generator = AstGenerator::default();

    let to_write_manually = HashSet::from([
        "PragmaStmt",
        "Offset",
        "FromClause",
        "TableOrSubquery",
        "JoinClause",
        "TypeName",
        "CaseWhenClause",
        "OpBetweenAnd",
        "OpNotBetweenAnd",
        "SignedNumber",
    ]);

    // Generation special case
    let bin_op_structs = HashSet::from([
        "OpConcat",
        "OpExtractOne",
        "OpExtractTwo",
        "OpMultiply",
        "OpDivide",
        "OpModulus",
        "OpAdd",
        "OpSubtract",
        "OpBinAnd",
        "OpBinOr",
        "OpBinLShift",
        "OpBinRShift",
        "OpLT",
        "OpGT",
        "OpLTE",
        "OpGTE",
        "OpEq",
        "OpNotEq",
        "OpAnd",
        "OpOr",
        "OpMatch",
        "OpRegexp",
        "OpGlob",
        "OpIs",
        "OpIsDistinctFrom",
        "OpIsNotDistinctFrom",
        "OpNotMatch",
        "OpNotRegexp",
        "OpNotGlob",
        "OpIsNot",
        "OpLike",
    ]);

    for node_data in UNGRAMMAR.nodes() {
        if to_write_manually.contains(node_data.name.as_str())
            || bin_op_structs.contains(node_data.name.as_str())
        {
            continue;
        }
        if matches!(&node_data.rule, Rule::Alt(_)) {
            generator.add_rust_enum(&node_data.name, &node_data.rule, true);
        } else {
            generator.add_rust_struct(&node_data.name, &node_data.rule);
        }
    }

    let skip_set = to_write_manually.union(&bin_op_structs).copied().collect();
    let code = write_rust_code(&generator, skip_set, bin_op_structs);

    let code_str = prettyplease::unparse(&syn::parse2(code).unwrap());

    println!("{code_str}");
}

impl AstGenerator {
    fn add_rust_struct(&mut self, name: &str, rule: &Rule) {
        let name = name.to_owned();
        let mut methods: Vec<RustMethod> = Vec::new();

        let mut rules = vec![rule];

        while let Some(rule) = rules.pop() {
            match rule {
                Rule::Labeled { .. } | Rule::Node(_) => {
                    let method = self.rule_to_rust_method(rule);
                    if method.search_kind == SearchKind::Rep {
                        if let Some(idx) = methods
                            .iter()
                            .position(|it| it.return_ty == method.return_ty)
                        {
                            assert!(methods[idx].search_kind == SearchKind::Node);
                            methods.remove(idx);
                        }
                    }
                    methods.push(method);
                }
                Rule::Token(_) => continue,
                Rule::Seq(vec) => {
                    rules.extend(vec.iter().rev());
                }
                Rule::Opt(rule) => rules.push(rule),
                _ => unreachable!("{} cannot be a rust struct", rule_to_str(rule)),
            }
        }

        // Return types are unique (to avoid ambiguity)
        if !methods.iter().map(|it| &it.return_ty).all_unique() {
            panic!("Methods not unique: {methods:?}");
        }

        let struct_ = RustStruct {
            name: name.clone(),
            methods,
        };
        // dbg!(&struct_);

        self.structs.insert(name.clone(), struct_);
    }

    fn rule_to_rust_method(&mut self, rule: &Rule) -> RustMethod {
        match rule {
            Rule::Node(node_id) => RustMethod {
                name: UNGRAMMAR
                    .get_node(*node_id)
                    .name
                    .to_owned()
                    .to_case(Case::Snake),
                search_kind: SearchKind::Node,
                return_ty: UNGRAMMAR.get_node(*node_id).name.to_owned(),
            },
            Rule::Labeled { label, rule } => {
                let name = label.to_owned();
                let search_kind;
                let return_ty;

                let rule = remove_opt(rule);

                match rule {
                    Rule::Node(node_id) => {
                        search_kind = SearchKind::Node;
                        return_ty = UNGRAMMAR.get_node(*node_id).name.to_owned();
                    }
                    Rule::Token(token_id) => {
                        let token_name = UNGRAMMAR.get_token(*token_id);
                        if token_name.starts_with("$") {
                            search_kind = SearchKind::TokenSet;
                        } else {
                            search_kind = SearchKind::Token;
                        }
                        return_ty = token_name.to_owned();
                    }
                    Rule::Alt(_) => {
                        search_kind = SearchKind::Alt;
                        return_ty = name.to_case(Case::UpperCamel);
                        self.add_rust_enum(&name, rule, false)
                    }
                    Rule::Rep(rule) => {
                        search_kind = SearchKind::Rep;

                        let rule = remove_opt(rule);
                        match rule {
                            Rule::Node(node_id) => {
                                return_ty = UNGRAMMAR.get_node(*node_id).name.to_owned();
                            }
                            Rule::Seq(vec) => {
                                let Some(Rule::Node(node_id)) =
                                    vec.iter().find(|it| matches!(it, Rule::Node(_)))
                                else {
                                    unreachable!(
                                        "Rep node with Seq as child must have node as child"
                                    )
                                };

                                return_ty = UNGRAMMAR.get_node(*node_id).name.to_owned();
                            }
                            _ => unreachable!("Label -> Rep -> (Node | Seq) only"),
                        }
                    }
                    _ => unreachable!("{} cannot be a rust method", rule_to_str(rule)),
                }

                RustMethod {
                    name,
                    search_kind,
                    return_ty,
                }
            }
            _ => unreachable!("{} cannot be a rust method", rule_to_str(rule)),
        }
    }

    fn add_rust_enum(&mut self, name: &str, rule: &Rule, is_node: bool) {
        let Rule::Alt(rules) = rule else {
            unreachable!("rust enum can only be made from alt rules")
        };

        let fields: Vec<_> = rules
            .iter()
            .map(|it| match it {
                Rule::Token(token_id) => {
                    let token_name = UNGRAMMAR.get_token(*token_id);
                    assert!(!token_name.starts_with("$"));

                    RustEnumField {
                        kind: RustEnumFieldKind::Token,
                        name: token_name.to_owned(),
                    }
                }
                Rule::Node(node_id) => {
                    let name = UNGRAMMAR.get_node(*node_id).name.to_owned();

                    RustEnumField {
                        kind: RustEnumFieldKind::Node,
                        name,
                    }
                }
                _ => unreachable!(
                    "Cannot make rust enum fields from anything other than Node | Token"
                ),
            })
            .collect();

        let name = name.to_case(Case::UpperCamel);

        if let Some(enum_) = self.enums.values().find(|it| it.name == name) {
            if !enum_.fields.iter().eq(fields.iter()) {
                panic!(
                    "Two different enums with same name: {} found: \nleft {:?}\n right: {:?}",
                    name, enum_.fields, fields
                )
            }
        } else {
            let enum_ = RustEnum {
                name: name.clone(),
                fields,
                is_non_terminal_node: is_node,
            };
            // dbg!(&enum_);
            self.enums.insert(name.clone(), enum_);
        }
    }
}

fn write_rust_code(
    generator: &AstGenerator,
    skip_set: HashSet<&str>,
    special_cases: HashSet<&str>,
) -> proc_macro2::TokenStream {
    let enums = generator.enums.values().map(|it| {
        let enum_name = format_ident!("{}", it.name);
        let fields = it.fields.iter().map(|field| {
            let variant_name = format_ident!("{}", convert_symbol_tokens(&field.name).unwrap_or(&field.name));

            match field.kind {
                RustEnumFieldKind::Node => quote! { #variant_name(#variant_name<'a>) },
                RustEnumFieldKind::Token => quote! { #variant_name(&'a SqliteToken) },
            }
        });

        let field_match_snippet = it.fields.iter().map(|field| {
            let variant_name = format_ident!("{}", convert_symbol_tokens(&field.name).unwrap_or(&field.name));
            let node_ident_name = if it.is_non_terminal_node {
                quote! {child}
            } else {
                quote! {node}
            };
            match field.kind {
                RustEnumFieldKind::Node => {
                    quote! {
                        CstNodeData::Tree(TreeKind::#variant_name) => return Some(Self::#variant_name(#variant_name::cast(#node_ident_name)?))
                    }
                },
                RustEnumFieldKind::Token => {
                    quote! {
                        CstNodeData::Token(token @ SqliteToken { kind: TokenKind::#variant_name, .. }) => return Some(Self::#variant_name(&token))
                    }
                },
            }
        });

        let field_method_snippet = it.fields.iter().map(|field| {
            let variant_name = format_ident!("{}", convert_symbol_tokens(&field.name).unwrap_or(&field.name));
            let variant_name_lower = format_ident!("{}", convert_symbol_tokens(&field.name).unwrap_or(&field.name).to_case(Case::Snake));

            match field.kind {
                RustEnumFieldKind::Node => {
                    quote! {
                        pub fn #variant_name_lower(self) -> Option<#variant_name<'a>> {
                            match self {
                                Self::#variant_name(item) => Some(item),
                                _ => None
                            }
                        }
                    }
                },
                RustEnumFieldKind::Token => {
                    quote! {
                        pub fn #variant_name_lower(self) -> Option<&'a SqliteToken> {
                            match self {
                                Self::#variant_name(item) => Some(item),
                                _ => None
                            }
                        }
                    }
                },
            }
        });

        if it.is_non_terminal_node {
            quote! {
                pub enum #enum_name<'a> {
                    #(#fields ,)*
                }

                impl<'a> #enum_name<'a> {
                    pub fn cast(node: CstNode<'a>) -> Option<Self> {
                        use SqliteTreeKind as TreeKind;
                        use SqliteTokenKind as TokenKind;
                        if node.tree() != Some(TreeKind::#enum_name) {
                            return None
                        }

                        node.valid_children().flat_map(|child| match child.data {
                            #(#field_match_snippet,)*
                            _ => return None
                        }).next()
                    }

                    #(#field_method_snippet)*
                }
            }
        } else {
            quote! {
                pub enum #enum_name<'a> {
                    #(#fields ,)*
                }

                impl<'a> #enum_name<'a> {
                    pub fn cast(node: CstNode<'a>) -> Option<Self> {
                        use SqliteTreeKind as TreeKind;
                        use SqliteTokenKind as TokenKind;
                        match node.data {
                            #(#field_match_snippet,)*
                            _ => return None
                        }
                    }

                    #(#field_method_snippet)*
                }
            }
        }
    });

    let structs = generator.structs.values().map(|it| {
        let struct_name = format_ident!("{}", it.name);

        let methods = it.methods.iter().map(|method| {
            let method_name = format_ident!("{}", method.name);
            match method.search_kind {
                SearchKind::Node => {
                    let return_ty = format_ident!("{}", method.return_ty);
                    quote! {
                        pub fn #method_name(&self) -> Option<#return_ty<'a>> {
                            self
                                .inner
                                .find_children(SqliteTreeKind::#return_ty)
                                .flat_map(#return_ty::cast)
                                .next()
                        }
                    }
                }
                SearchKind::TokenSet => {
                    quote! {
                        pub fn #method_name(&self) -> Option<&'a SqliteToken> {
                            self
                                .inner
                                .valid_children()
                                .next()
                                .and_then(|it| it.token())
                        }
                    }
                }
                SearchKind::Token => {
                    let return_ty = format_ident!("{}", method.return_ty);
                    quote! {
                        pub fn #method_name(&self) -> Option<&'a SqliteToken> {
                            self
                                .inner
                                .find_children(SqliteTokenKind::#return_ty)
                                .map(|it| it.token().unwrap())
                                .next()
                        }
                    }
                }
                SearchKind::Rep => {
                    let return_ty = format_ident!("{}", method.return_ty);

                    if generator.structs.contains_key(&method.return_ty)
                        || skip_set.contains(method.return_ty.as_str())
                    {
                        quote! {
                            pub fn #method_name(&self) -> impl Iterator<Item = #return_ty<'a>> {
                                self.inner
                                    .valid_children()
                                    .flat_map(#return_ty::cast)
                            }
                        }
                    } else if generator.enums.contains_key(&method.return_ty) {
                        let is_node = generator
                            .enums
                            .get(&method.return_ty)
                            .unwrap()
                            .is_non_terminal_node;

                        let cast_code = if is_node {
                            quote! {#return_ty::cast}
                        } else {
                            quote! { #return_ty::cast }
                        };

                        quote! {
                            pub fn #method_name(&self) -> impl Iterator<Item = #return_ty<'a>> {
                                self.inner
                                    .find_children(SqliteTreeKind::#return_ty)
                                    .flat_map(|it| it.children())
                                    .flat_map(#cast_code)
                            }
                        }
                    } else {
                        unreachable!("Unknown struct/enum: {}", method.return_ty);
                    }
                }
                SearchKind::Alt => {
                    let return_ty = format_ident!("{}", method.return_ty);
                    quote! {
                        pub fn #method_name(&self) -> Option<#return_ty<'a>> {
                            self
                               .inner
                               .children()
                               .flat_map(#return_ty::cast)
                               .next()
                       }
                    }
                }
            }
        });

        quote! {
            pub struct #struct_name<'a> { pub inner: CstNode<'a> }

            impl<'a> #struct_name<'a> {
                pub fn cast(node: CstNode<'a>) -> Option<Self> {
                    if node.tree() == Some(SqliteTreeKind::#struct_name) {
                        Some(Self{ inner: node })
                    } else {
                        None
                    }
                }
                #(#methods)*
            }

        }
    });

    let special_case_structs = special_cases.iter().map(|it| {
        let struct_name = format_ident!("{it}");

        quote! {
            pub struct #struct_name<'a> { inner: CstNode<'a>  }

            impl<'a> #struct_name<'a> {
                pub fn cast(node: CstNode<'a>) -> Option<Self> {
                    if node.tree() == Some(SqliteTreeKind::#struct_name) {
                        Some(Self{ inner: node })
                    } else {
                        None
                    }
                }

                pub fn lhs_expr(&self) -> Option<Expr<'a>> {
                    self.inner.valid_children().next().and_then(Expr::cast)
                }

                pub fn rhs_expr(&self) -> Option<Expr<'a>> {
                    let mut child_iter = self.inner.valid_children();

                    // Navigate to operand, which will let us skip lhs expr
                    child_iter.find(|it| it.tree() != Some(SqliteTreeKind::Expr));

                    // Now navigate to navigate to next Expr node, which should be rhs
                    child_iter.find_map(Expr::cast)
                }
            }
        }
    });

    quote::quote! {
        #![allow(non_camel_case_types)]

        pub use crate::{NodeId, CstNode, SqliteTokenKind, SqliteTreeKind, SqliteToken, CstNodeData};
        use crate::grammar::common::*;
        use super::manual::*;

        #(#enums)*

        #(#structs)*

        #(#special_case_structs)*
    }
}

fn remove_opt(mut rule: &Rule) -> &Rule {
    while let Rule::Opt(inner) = rule {
        rule = inner
    }

    rule
}

fn convert_symbol_tokens(tk: &str) -> Option<&str> {
    match tk {
        "*" => Some("STAR"),
        ";" => Some("SEMICOLON"),
        "," => Some("COMMA"),
        "." => Some("DOT"),
        "+" => Some("PLUS"),
        "-" => Some("MINUS"),
        "/" => Some("F_SLASH"),
        "||" => Some("DOUBLE_PIPE"),
        "->" => Some("EXTRACT_ONE"),
        "->>" => Some("EXTRACT_TWO"),
        "<>" => Some("NOT_EQ_SQL"),
        "%" => Some("PERCENT"),
        "&" => Some("AMPERSAND"),
        "|" => Some("PIPE"),
        "<<" => Some("L_CHEV_TWO"),
        ">>" => Some("R_CHEV_TWO"),
        "<" => Some("L_CHEV"),
        ">" => Some("R_CHEV"),
        "<=" => Some("L_CHEV_EQ"),
        ">=" => Some("R_CHEV_EQ"),
        "==" => Some("EQ"),
        "=" => Some("EQ_SQL"),
        "!=" => Some("NOT_EQ"),
        "~" => Some("TILDA"),
        "?" => Some("Q_MARK"),
        "@" => Some("AT_MARK"),
        ":" => Some("COLON"),
        "(" => Some("L_PAREN"),
        ")" => Some("R_PAREN"),
        _ => None,
    }
}
