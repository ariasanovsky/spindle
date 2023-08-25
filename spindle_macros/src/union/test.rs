use quote::ToTokens;
// use spindle_db::TypeDb;
use syn::parse_quote;

use crate::{union::{RawSpinInput, NewUnion}, case::{UpperCamelIdent, PrimitiveIdent}};

#[test]
fn parse_a_new_union_of_primitives() {
    let input = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let new_union = spin_input._new_union().unwrap();
    let NewUnion { ident, primitives } = new_union;
    let UpperCamelIdent(ident) = ident;
    assert_eq!(ident.to_string(), "U");
    let primitives = primitives.iter().map(|primitive| {
        let PrimitiveIdent(primitive) = primitive;
        primitive.to_string()
    }).collect::<Vec<_>>();
    assert_eq!(primitives, vec!["f32", "u64"]);
}

#[test]
fn parse_an_old_union() {
    let input = quote::quote! {
        V
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let spin_input = spin_input._union_in_scope().unwrap();
    let ident = &spin_input.0 .0;
    assert_eq!(ident.to_string(), "V");
}

#[test]
fn emit_tokens_from_new_union() {
    let input = quote::quote! {
        U = f32 | u64
    };

    let spin_input: RawSpinInput = parse_quote!(#input);
    let new_union = spin_input._new_union().unwrap();
    let expected_tokens = quote::quote! {
        #[repr(C)]
        pub union U {
            _0: f32,
            _1: u64,
        }
    };
    assert_eq!(new_union.to_token_stream().to_string(), expected_tokens.to_string());

    // let uuid = &db_union.uuid;
    // let impl_uuid = db_union.impl_uuid();
    // let impl_uuid_2 = quote::quote! {
    //     unsafe impl spindle::__db::DbUuid for U {
    //         const __UUID: &'static str = #uuid;
    //     }
    // };
    // assert_eq!(impl_uuid.to_string(), impl_uuid_2.to_string());
    // let impl_device_repr = db_union.impl_device_repr();
    // let impl_device_repr_2 = quote::quote! {
    //     unsafe impl spindle::__cudarc::DeviceRepr for U {}
    // };
    // assert_eq!(impl_device_repr.to_string(), impl_device_repr_2.to_string());

    // let impl_raw_converts = db_union.impl_raw_converts();
    // let unpacked_impls = quote::quote! {
    //     #(#impl_raw_converts)*
    // };
    // let impl_raw_converts_2 = quote::quote! {
    //     unsafe impl spindle::__union::RawConvert<f32> for U {}
    //     unsafe impl spindle::__union::RawConvert<u64> for U {}
    // };
    // assert_eq!(unpacked_impls.to_string(), impl_raw_converts_2.to_string());
}
