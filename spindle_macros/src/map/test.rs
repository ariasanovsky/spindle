use proc_macro2::TokenStream;
use syn::parse_quote;

use crate::map::MapFn;

#[test]
fn add_univariate_pure_function_to_db() {
    let input: TokenStream = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let input: MapFn = parse_quote!(#input);
    dbg!();
    todo!("{input}");
}