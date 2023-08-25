use proc_macro2::TokenStream;
// use serde::{Deserialize, Serialize};
use spin::SpinInputs;
use syn::parse_macro_input;

// todo! deprecate
// mod basic_range;
pub(crate) mod case;
pub(crate) mod db;
pub(crate) mod dev_item_fn;
// todo! ?deprecate
pub(crate) mod error;
pub(crate) mod init;
pub(crate) mod file_strings;
pub(crate) mod map_fn;
pub(crate) mod regulate;
pub(crate) mod spin;
pub(crate) mod tag;
pub(crate) mod union;

// todo! deprecate
type TokenResult = Result<TokenStream, TokenStream>;

// todo! deprecate
#[derive(Clone)]
struct BasicRangeAttrs;

// todo! deprecate
// #[derive(Clone)]
// struct BasicRangeFn(syn::ItemFn);

#[proc_macro_attribute]
pub fn init(
    attr: proc_macro::TokenStream,
    init_map: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use init::{Attrs, DevInitFn};
    let attrs: Attrs = parse_macro_input!(attr as Attrs);
    let init_map: DevInitFn = parse_macro_input!(init_map as DevInitFn);
    let result: proc_macro2::TokenStream = init::init(attrs, init_map);
    result.into()
}

// todo! deprecate
// #[derive(Debug, Serialize, Deserialize)]
// struct RangeSpindle {
//     home: String,
//     name: String,
//     populated: bool,
//     compiled: bool,
//     device: Option<String>,
//     msg: Option<String>,
//     kernel: Option<String>,
// }

#[proc_macro]
pub fn spin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let inputs: SpinInputs = syn::parse_macro_input!(input as SpinInputs);
    spin::spin(inputs, "types")
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// todo! deprecate
// #[proc_macro_attribute]
// pub fn basic_range(
//     attr: proc_macro::TokenStream,
//     item: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     let attr = parse_macro_input!(attr as BasicRangeAttrs);
//     let item = parse_macro_input!(item as BasicRangeFn);
//     let result = emit_range_kernel(attr, item);
//     into_token_stream(result)
// }

#[proc_macro_attribute]
pub fn map(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use map_fn::{MapAttrs, DevMapFn};
    let attr = parse_macro_input!(attr as MapAttrs);
    let map_fn = parse_macro_input!(item as DevMapFn);
    map_fn::map(attr, map_fn, "types")
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// todo! deprecate
// fn into_token_stream(result: TokenResult) -> proc_macro::TokenStream {
//     match result {
//         Ok(result) | Err(result) => result,
//     }
//     .into()
// }

// todo! deprecate
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

// todo! deprecate
// todo! write a test for this
pub(crate) fn snake_to_camel(s: &str) -> String {
    let s = s.split('_').map(camel_word).collect::<Vec<_>>();
    s.join("")
}
