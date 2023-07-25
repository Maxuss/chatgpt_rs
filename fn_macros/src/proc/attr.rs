use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use syn::ItemFn;
use proc_macro::TokenStream;

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

    todo!()
}