use syn::{Token, parse_quote};

use crate::{basic_range::parse, init::Attrs};

#[test]
fn example_02_range_init() {
    let pound: Token![#] = Default::default();
    // test the attr parsing
    let attr = quote::quote! {
        #pound example_02_test
    };
    let attrs: Attrs = parse_quote! {
        #attr
    };
    let expected_attrs = todo!();
    assert_eq!(atrrs, expected_attrs);

    // test the function parsing
    let item = quote::quote! {
        fn square_over_two(x: u64) -> f32 {
            ((x * x) / 2) as f32
        }
    };
    let item: InputInitFn = parse_quote! {
        #item
    };
    let expected_item = todo!();
    assert_eq!(item, expected_item);
    todo!("what else to test???")
}

