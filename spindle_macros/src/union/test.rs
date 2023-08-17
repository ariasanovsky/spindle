use spindle_db::{TypeDb, union::DbUnion};
use syn::parse_quote;

use crate::union::{RawSpinInput, tokens::UnionTokens};

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
    let ident = spin_input.0.0;
    assert_eq!(ident.to_string(), "U");
    let fields: Vec<String> =
        spin_input
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
    let ident = &spin_input.0.0;
    assert_eq!(ident.to_string(), "V");
}

#[test]
fn insert_a_new_union_to_the_db() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "insert_a_new_union_to_the_db";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();
    
    // parse a union & insert it into the db
    let input = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let spin_input = spin_input._new_union().unwrap();
    let db_union: DbUnion = db.get_or_insert_union(spin_input).unwrap();
}

#[test]
fn get_an_old_union_from_the_db() {
    todo!("useless test?");
    // // connect to database
    // // add function to database
    // const DB_NAME: &str = "get_an_old_union_from_the_db";
    // const DB_PATH: &str = "target/spindle/db/";
    // let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();

    // // parse a union & insert it into the db
    // let pound: syn::Token![#] = Default::default();
    // let input = quote::quote! {
    //     #pound example, U = f32 | u64
    // };
    // let spin_input: RawSpinInput = parse_quote!(#input);
    // let spin_input = spin_input._new_union().unwrap();
    // let db_union: DbUnion = db.get_or_insert_union(spin_input).unwrap();

    // struct U;
    // trait DbUuid {
    //     const __UUID: &'static str;
    // }

    // impl DbUuid for U {
    //     const __UUID: &'static str = "asdf";
    // }

    // // parse the same union & get it from the db
    // let input = quote::quote! { U };
    // let spin_input: RawSpinInput = parse_quote!(#input);
    // /*
    // * we use small names for unions (e.g. `U`)
    //     - `U` appears in the db frequently
    // * eliding the fields of a union is good ergonomics
    //     - we need to find the fields of `U`
    // * the db doesn't encode scope
    // * as a compromise, get fields from scope,
    //     - e.g., `let fields: Vec<_> = U::__fields();`
    //     - this is mildly unsanitary
    //     - possibly we can do a const eval macro hack
    // * syn has some dundermethods, we're in good company
    // * the previously parsed union is in the db
    // * so we can assume that
    // impl U {
    //     fn __fields() -> Vec<String> {
    //         vec!["f32", "u64"]
    //     }
    // }
    // exists in scope
    // */
    // // let uuid: String = db_union.uuid.clone(); // U::__UUID.to_string()
    // // let db_uuid_2 = db.get_union_from_uuid_and_ident(uuid, spin_input.ident().to_string()).unwrap();
    // // assert_eq!(db_union, db_uuid_2);
    // todo!()
}

#[test]
fn emit_tokens_from_new_union() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "emit_tokens_from_new_union";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();
    
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
