use spindle_db::{TypeDb, union::DbUnion};
use syn::parse_quote;

use crate::{map::MapFn, spin::{RawSpinInput, RawSpinInputs}};

#[test]
fn spin_parses_union_in_scope_and_new_and_a_map_in_scope() {
    let input = quote::quote! {
        U = f32 | u64, V, foo,
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
fn spin_gets_existing_map_from_db() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "get_existing_map_from_db";
    const DB_PATH: &str = "target/spindle/db/";
    todo!();
}

#[test]
fn spin_gets_crate_from_one_union_and_one_map() {
    todo!("we're now adding maps to the spin syntax");
    // // connect to database
    // // add function to database
    // const DB_NAME: &str = "spin_gets_crate_from_one_union_and_one_map";
    // const DB_PATH: &str = "target/spindle/db/";
    // let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();

    // // parse and insert a map into the db
    // let map = quote::quote! {
    //     fn foo(x: u64) -> f32 {
    //         x as f32
    //     }
    // };
    // let map: MapFn = parse_quote!(#map);
    // dbg!(&map);
    // let db_map = db.get_or_insert_map(&map).unwrap();
    // dbg!(&db_map);

    // // parse and insert a union into the db
    // let union = quote::quote! {
    //     U = f32 | u64
    // };
    // let spin_input: RawSpinInput = parse_quote!(#union);
    // let db_union: DbUnion = db.get_or_insert_union(&spin_input).unwrap();
    // dbg!(&db_union);

    // // get the crate
    // let db_crate = db.get_or_insert_crate_from_unions(vec![db_union]).unwrap();
    // dbg!(&db_crate);
    // assert_eq!(db_crate.unions.len(), 1);
}
