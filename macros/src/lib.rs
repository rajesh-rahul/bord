extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Result, Token};
#[derive(Debug)]
struct MyMacroInput {
    result_type: Option<syn::TypePath>,
    query_string: syn::LitStr,
    params: Vec<syn::Ident>,
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let result_type: Option<syn::TypePath> = input.parse().ok();
        if result_type.is_some() {
            input.parse::<Token![,]>()?;
        }

        let query_string: syn::LitStr = input.parse()?;

        let _ = input.parse::<Token![,]>();

        let params: Punctuated<syn::Ident, Token![,]> =
            input.parse_terminated(syn::Ident::parse, Token![,])?;

        let params: Vec<_> = params.into_iter().collect();

        Ok(MyMacroInput {
            result_type,
            query_string,
            params,
        })
    }
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MyMacroInput);
    let sql = &input.query_string;

    quote! {
      1
    }
    .into()
}
