use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use std::collections::HashMap;
use syn::{Expr, ItemFn, Meta::NameValue, MetaNameValue, Lit, ExprLit};

#[derive(Debug, FromMeta)]
struct FunctionProps {
    #[darling(default)] // async by default
    sync: bool,
}

pub fn process_fn_macro(
    attr: TokenStream,
    function: TokenStream
) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    let input = syn::parse_macro_input!(function as ItemFn);

    let props = match FunctionProps::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let docs = extract_docs(&input);

    todo!()
}

fn extract_docs(input: &ItemFn) -> Option<FunctionDocs> {
    let docs = input.attrs.iter().filter_map(|attr| {
        if !attr.path().is_ident("doc") {
            return None
        }

        let meta = &attr.meta;

        if let NameValue(MetaNameValue { value: Expr::Lit(ExprLit { lit: Lit::Str(str), .. }), .. }) = meta {
            return Some(str.value())
        }

        None
    }).collect::<Vec<String>>();

    let mut parameter_docs = HashMap::with_capacity(4);

    let mut lines = docs
        .iter()
        .flat_map(|a| a.split("\n"))
        .map(str::trim)
        .skip_while(|s| s.is_empty())
        .filter_map(|line| {
            if line.starts_with('*') && line.contains('-') {
                let (param_name, param_doc) = line[1..].trim().split_once('-').unwrap();
                let param_name = param_name.trim();
                let param_doc = param_doc.trim();
                parameter_docs.insert(param_name.to_owned(), param_doc.to_owned());
                None
            } else {
                Some(line)
            }
        })
        .collect::<Vec<_>>();

    if let Some(&"") = lines.last() {
        lines.pop();
    }

    let joined = lines.join("\n");

    if joined.is_empty() {
        None
    } else {
        Some(FunctionDocs {
            description: joined,
            parameter_docs: parameter_docs
        })
    }
}

#[derive(Debug, Clone)]
struct FunctionDocs {
    description: String,
    parameter_docs: HashMap<String, String>, // todo: maybe hashmap can be replaced with something more efficient
}