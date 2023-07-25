use proc_macro::TokenStream;

mod proc;

#[proc_macro_attribute]
pub fn gpt_function(attr: TokenStream, value: TokenStream) -> TokenStream {
    proc::process_fn_macro(attr, value)
}