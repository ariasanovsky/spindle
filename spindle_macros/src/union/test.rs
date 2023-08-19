use spindle_db::{union::DbUnion, TypeDb};
use syn::parse_quote;

use crate::union::{tokens::UnionTokens, RawSpinInput};

#[test]
fn parse_a_new_union_of_primitives() {
    let input = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let spin_input = if let RawSpinInput::NewUnion(spin_input) = spin_input {
        spin_input
    } else {
        panic!("expected a new union");
    };
    let ident = spin_input.0 .0;
    assert_eq!(ident.to_string(), "U");
    let fields: Vec<String> = spin_input
        .1
        .iter()
        .map(|field| field.0.to_string())
        .collect();
    assert_eq!(fields, vec!["f32", "u64"]);
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
fn insert_a_new_union_to_the_db() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "insert_a_new_union_to_the_db";
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let db_path = format!("{target}/spindle/db/");
    let db = TypeDb::new(DB_NAME, db_path).unwrap();

    // parse a union & insert it into the db
    let input = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let spin_input = spin_input._new_union().unwrap();
    let _db_union: DbUnion = db.get_or_insert_union(spin_input).unwrap();
}

#[test]
fn emit_tokens_from_new_union() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "emit_tokens_from_new_union";
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let db_path = format!("{target}/spindle/db/");
    let db = TypeDb::new(DB_NAME, db_path).unwrap();

    let input = quote::quote! {
        U = f32 | u64
    };

    let spin_input: RawSpinInput = parse_quote!(#input);
    let spin_input = spin_input._new_union().unwrap();
    let db_union: DbUnion = db.get_or_insert_union(spin_input).unwrap();
    let decl = db_union.declaration();
    let decl_2 = quote::quote! {
        #[repr(C)]
        union U {
            _0: f32,
            _1: u64,
        }
    };
    assert_eq!(decl.to_string(), decl_2.to_string());

    let uuid = &db_union.uuid;
    let impl_uuid = db_union.impl_uuid();
    let impl_uuid_2 = quote::quote! {
        unsafe impl spindle::__db::DbUuid for U {
            const __UUID: &'static str = #uuid;
        }
    };
    assert_eq!(impl_uuid.to_string(), impl_uuid_2.to_string());
    let impl_device_repr = db_union.impl_device_repr();
    let impl_device_repr_2 = quote::quote! {
        unsafe impl spindle::__cudarc::DeviceRepr for U {}
    };
    assert_eq!(impl_device_repr.to_string(), impl_device_repr_2.to_string());

    let impl_raw_converts = db_union.impl_raw_converts();
    let unpacked_impls = quote::quote! {
        #(#impl_raw_converts)*
    };
    let impl_raw_converts_2 = quote::quote! {
        unsafe impl spindle::__union::RawConvert<f32> for U {}
        unsafe impl spindle::__union::RawConvert<u64> for U {}
    };
    assert_eq!(unpacked_impls.to_string(), impl_raw_converts_2.to_string());
}
