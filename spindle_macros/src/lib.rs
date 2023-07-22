use basic_range::emit_range_kernel;
use map::serialize_map;
use proc_macro2::TokenStream;
use serde::{Deserialize, Serialize};
use spin::SpinInput;
use syn::parse_macro_input;

mod basic_range;
mod error;
pub(crate) mod file_strings;
mod map;
mod spin;

type TokenResult = Result<TokenStream, TokenStream>;

#[derive(Clone)]
struct BasicRangeAttrs;

#[derive(Clone)]
struct MapAttrs;

#[derive(Clone)]
struct BasicRangeFn(syn::ItemFn);

#[derive(Clone)]
struct MapFn(syn::ItemFn);

#[derive(Debug, Serialize, Deserialize)]
struct RangeSpindle {
    home: String,
    name: String,
    populated: bool,
    compiled: bool,
    device: Option<String>,
    msg: Option<String>,
    kernel: Option<String>,
}

#[proc_macro]
pub fn spin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as SpinInput);
    let union = input.union();
    let impls = input.impls();
    let expanded = quote::quote! {
        #union
        #impls
    };
    expanded.into()
}

#[proc_macro_attribute]
pub fn basic_range(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as BasicRangeAttrs);
    let item = parse_macro_input!(item as BasicRangeFn);
    let result = emit_range_kernel(attr, item);
    into_token_stream(result)
}

#[proc_macro_attribute]
pub fn map(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr as MapAttrs);
    let item = parse_macro_input!(item as MapFn);
    let result = serialize_map(attr, item);
    into_token_stream(result)
}

fn into_token_stream(result: TokenResult) -> proc_macro::TokenStream {
    match result {
        Ok(result) | Err(result) => result,
    }
    .into()
}

pub(crate) fn camel_word(s: &str) -> String {
    let mut chars = s.chars();
    let mut camel = if let Some(c) = chars.next() {
        String::from(c).to_uppercase()
    } else {
        return String::new();
    };
    chars.map(char::to_lowercase).for_each(|c| {
        camel = format!("{camel}{c}");
    });
    camel
}

pub(crate) fn snake_to_camel(s: &str) -> String {
    let s = s.split('_').map(camel_word).collect::<Vec<_>>();
    s.join("")
}
