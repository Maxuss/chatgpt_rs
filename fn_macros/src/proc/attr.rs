use proc_macro::TokenStream;
use std::collections::HashMap;
use proc_macro2::Literal;
use syn::{Expr, ItemFn, Meta::NameValue, MetaNameValue, Lit, ExprLit, FnArg, Pat, PatType, LitStr, Type, ReturnType, TypePath, parse_quote, GenericParam};
use syn::spanned::Spanned;
use quote::{quote_spanned, quote};
use syn::Pat::Ident;
use syn::token::RArrow;

pub fn process_fn_macro(
    attr: TokenStream,
    function: TokenStream
) -> TokenStream {
    let input = syn::parse_macro_input!(function as ItemFn);

    let docs = extract_docs(&input);

    if let None = docs {
        return syn::Error::new(input.span(), "This function does not have description. Make sure to add documentation to your ChatGPT functions.").into_compile_error().into()
    }

    if let None = &input.sig.asyncness {
        return syn::Error::new(input.sig.span(), "ChatGPT functions are required to be async.").into_compile_error().into()
    }

    let docs = docs.unwrap();

    let sig = rebuild_fn_sig(&input);
    let args_struct = build_arguments_struct(&input, &docs);
    let callable_struct = build_callable_struct(&input);
    let descriptor = build_function_descriptor(&input, &docs);
    (quote_spanned!(input.span() =>
        #args_struct
        #callable_struct

        #sig {
            use chatgpt::functions::*;

            #descriptor
        })).into()
}

fn build_callable_struct(input: &ItemFn) -> proc_macro2::TokenStream {
    let deconstructed_args = deconstruct_args(input);
    let body = &input.block;
    let fname = &input.sig.ident;
    let name = syn::Ident::new(&format!("__{fname}_Function"), input.sig.ident.span());
    let aname = syn::Ident::new(&format!("__{fname}_FunctionArguments"), input.sig.ident.span());

    quote_spanned!(input.span() =>
        use chatgpt::functions::async_trait::async_trait as __async_trait;

        #[doc(hidden)]
        #[derive(Debug, Copy, Clone)]
        struct #name;

        #[__async_trait]
        impl chatgpt::functions::CallableAsyncFunction<#aname> for #name {
            async fn invoke(arguments: #aname) {
                #deconstructed_args
                #body
            }
        }
    )
}

fn build_function_descriptor(input: &ItemFn, docs: &FunctionDocs) -> proc_macro2::TokenStream {
    let fn_name = &input.sig.ident;
    let description = syn::Lit::Str(LitStr::new(&docs.description, input.sig.span()));
    let fname = &input.sig.ident;
    let aname = syn::Ident::new(&format!("__{fname}_FunctionArguments"), input.sig.ident.span());
    let cname = syn::Ident::new(&format!("__{fname}_Function"), input.sig.ident.span());
    quote_spanned!(input.span() =>
        GptFunction {
            descriptor: FunctionDescriptor {
                name: stringify!(#fn_name),
                description: #description,
                parameters: core::marker::PhantomData::<#aname>::default()
            },
            callable: core::marker::PhantomData::<#cname>::default()
        }
    )
}

fn build_arguments_struct(input: &ItemFn, docs: &FunctionDocs) -> proc_macro2::TokenStream {
    let args = prepare_struct_args(input, docs);
    let name = &input.sig.ident;
    let name = syn::Ident::new(&format!("__{name}_FunctionArguments"), input.sig.ident.span());
    quote! {
        use chatgpt::functions::schema as schemars;

        #[allow(non_camel_case_types)]
        #[allow(missing_docs)]
        #[derive(chatgpt::functions::serde::Deserialize, chatgpt::functions::schema::JsonSchema, Debug, Clone)]
        #[doc(hidden)]
        pub struct #name {
            #args
        }
    }
}

fn prepare_struct_args(input: &ItemFn, docs: &FunctionDocs) -> proc_macro2::TokenStream {
    let deconstructed = deconstruct_args_into_struct(input);
    let span = input.span();
    let fields = deconstructed.into_iter().map(|(name, ty, stream)| {
        let docs = docs.parameter_docs.get(&name).unwrap_or(&name);
        let literal = syn::Lit::Str(LitStr::new(docs, span));
        let doc_attr = quote_spanned!(span => #[doc = #literal]);
        quote_spanned!(span => #doc_attr #stream: #ty)
    });
    quote_spanned!(span => #(#fields),*)
}

fn rebuild_fn_sig(input: &ItemFn) -> proc_macro2::TokenStream {
    let mut other_sig = input.sig.clone();
    let fname = &input.sig.ident;
    let name = syn::Ident::new(&format!("__{fname}_FunctionArguments"), input.sig.ident.span());
    let callable_name = syn::Ident::new(&format!("__{fname}_Function"), input.sig.ident.span());

    other_sig.asyncness = None;
    other_sig.constness = None;
    other_sig.generics.params.push(parse_quote!('a));
    other_sig.inputs.clear();
    other_sig.output = ReturnType::Type(RArrow::default(), parse_quote!(chatgpt::functions::GptFunction<'a, #name, #callable_name>));
    quote_spanned!(other_sig.span() => #other_sig)
}

fn deconstruct_args_into_struct(input: &ItemFn) -> Vec<(String, proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let args = input.sig.inputs.iter().filter_map(|each| {
        if let FnArg::Typed(typed) = each {
            let dec = deconstruct_single_pat_into_field(&typed.pat);
            let ty = &*typed.ty;
            Some((dec.0, quote_spanned!(ty.span() => #ty), dec.1))
        } else {
            None
        }
    }).collect::<Vec<(String, proc_macro2::TokenStream, proc_macro2::TokenStream)>>();
    args
}

fn deconstruct_single_pat_into_field(pat: &Pat) -> (String, proc_macro2::TokenStream) {
    match pat {
        Pat::Ident(ident) => {
            let ident = &ident.ident;
            (ident.to_string(), quote_spanned!(ident.span() => #ident))
        }
        Pat::Type(ty) => {
            let id = if let Pat::Ident(ident) = &*ty.pat {
                &ident.ident
            } else {
                return ("null".to_owned(), syn::Error::new(pat.span(), "Only typed arguments (e.g. `a: i32`) are supported by ChatGPT functions currently.").into_compile_error())
            };
            (id.to_string(), quote_spanned!(ty.span() => #ty))
        }
        other => ("null".to_owned(), syn::Error::new(pat.span(), format!("Pattern of type {other:?} is not supported for ChatGPT functions.\nOnly typed arguments (e.g. `a: i32`) are supported currently.")).into_compile_error())
    }
}

fn deconstruct_args(input: &ItemFn) -> proc_macro2::TokenStream {
    let args = input.sig.inputs.iter().filter_map(|each| {
        if let FnArg::Typed(typed) = each {
            Some(deconstruct_single_pat(&typed.pat))
        } else {
            None
        }
    });
    quote_spanned!(input.sig.inputs.span() => #(#args ;)*)
}

fn deconstruct_single_pat(pat: &Pat) -> proc_macro2::TokenStream {
    match pat {
        Pat::Type(ty) => {
            let id = if let Pat::Ident(ident) = &*ty.pat {
                &ident.ident
            } else {
                return syn::Error::new(pat.span(), "Only typed arguments (e.g. `a: i32`) are supported by ChatGPT functions currently.").into_compile_error()
            };
            quote_spanned!(ty.span() => let #ty = arguments.#id)
        }
        Pat::Ident(ident) => {
            let ident = &ident.ident;
            quote_spanned!(ident.span() => let #ident = arguments.#ident)
        }
        other => syn::Error::new(pat.span(), format!("Pattern of type {other:?} is not supported for ChatGPT functions.\nOnly typed arguments (e.g. `a: i32`) are supported currently.")).into_compile_error()
    }
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