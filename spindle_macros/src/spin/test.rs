use syn::parse_quote;

use crate::map::{MapAttrs, MapFn};

#[test]
fn example_01_test() {
    let pound = syn::token::Pound::default();
    let map_attr_input = quote::quote! {
        #pound example_01
    };
    let attrs: MapAttrs = parse_quote!(#map_attr_input);
    let expected_attrs = vec!["example_01"];
    assert_eq!(attrs._tags.into_iter().map(|attr| attr.0.0.to_string()).collect::<Vec<_>>(), expected_attrs);

    let map_fn_input = quote::quote! {
        fn i32_to_f64(x: i32) -> f64 {
            x as f64
        }
    };
    let map: MapFn = parse_quote!(#map_fn_input);

    let spin_input = quote::quote! {
        #pound example, U = f32 | u64,
    };
    todo!()
}
