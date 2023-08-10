use spindle_db::{TypeDb, union::DbUnion};
use syn::parse_quote;

use super::{RawSpinInput, RawSpinInputs};

#[test]
fn parse_a_new_union_of_primitives() {
    let input = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let ident = spin_input.ident();
    assert_eq!(ident.0.to_string(), "U");
    let fields: Vec<String> =
        spin_input
        .fields()
        .unwrap()
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
    let ident = spin_input.ident();
    assert_eq!(ident.0.to_string(), "V");
    let fields = spin_input.fields();
    assert!(fields.is_none());
}

#[test]
fn parse_an_old_union_and_a_new_union_of_primitives() {
    let input = quote::quote! {
        U = f32 | u64, V
    };
    let spin_inputs: RawSpinInputs = parse_quote!(#input);
    let u = spin_inputs.0.get(0).unwrap();
    let ident = u.ident();
    assert_eq!(ident.0.to_string(), "U");
    let fields: Vec<String> =
        u
        .fields()
        .unwrap()
        .iter()
        .map(|field| field.0.to_string())
        .collect();
    assert_eq!(fields, vec!["f32", "u64"]);
    let v = spin_inputs.0.get(1).unwrap();
    let ident = v.ident();
    assert_eq!(ident.0.to_string(), "V");
    let fields = v.fields();
    assert!(fields.is_none());
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
    let db_union: DbUnion = db.get_or_insert_union(&spin_input).unwrap();
    dbg!(&db_union);
}

#[test]
fn get_an_old_union_from_the_db() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "get_an_old_union_from_the_db";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();
    
    // parse a union & insert it into the db
    let input = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawSpinInput = parse_quote!(#input);
    let db_union: DbUnion = db.get_or_insert_union(&spin_input).unwrap();

    struct U;
    trait DbUuid {
        const __UUID: &'static str;
    }

    impl DbUuid for U {
        const __UUID: &'static str = "asdf";
    }

    dbg!(&db_union);

    // parse the same union & get it from the db
    let input = quote::quote! { U };
    let spin_input: RawSpinInput = parse_quote!(#input);
    /*
    * we use small names for unions (e.g. `U`)
        - `U` appears in the db frequently
    * eliding the fields of a union is good ergonomics
        - we need to find the fields of `U`
    * the db doesn't encode scope
    * as a compromise, get fields from scope,
        - e.g., `let fields: Vec<_> = U::__fields();`
        - this is mildly unsanitary
        - possibly we can do a const eval macro hack
    * syn has some dundermethods, we're in good company
    * the previously parsed union is in the db
    * so we can assume that
    impl U {
        fn __fields() -> Vec<String> {
            vec!["f32", "u64"]
        }
    }
    exists in scope
    */
    let uuid: String = db_union.uuid.clone(); // U::__UUID.to_string()
    dbg!(&uuid);
    let db_uuid_2 = db.get_union_from_uuid_and_ident(uuid, spin_input.ident().0.to_string()).unwrap();
    assert_eq!(db_union, db_uuid_2);
}