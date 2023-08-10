use spindle_db::{TypeDb, union::DbUnion};
use syn::parse_quote;

use crate::{map::MapFn, spin::{RawUnionInput, RawSpinInputs}};

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
fn get_crate_from_one_union_and_one_map() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "get_crate_from_one_union_and_one_map";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();

    // parse and insert a map into the db
    let map = quote::quote! {
        fn foo(x: u64) -> f32 {
            x as f32
        }
    };
    let map: MapFn = parse_quote!(#map);
    dbg!(&map);
    let db_map = db.get_or_insert_map(&map).unwrap();
    dbg!(&db_map);

    // parse and insert a union into the db
    let union = quote::quote! {
        U = f32 | u64
    };
    let spin_input: RawUnionInput = parse_quote!(#union);
    let db_union: DbUnion = db.get_or_insert_union(&spin_input).unwrap();
    dbg!(&db_union);

    // get the crate
    let db_crate = db.get_or_insert_crate_from_unions(vec![db_union]).unwrap();
    dbg!(&db_crate);
    assert_eq!(db_crate.unions.len(), 1);
}