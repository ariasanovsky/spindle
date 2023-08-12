use spindle_db::TypeDb;
use syn::parse_quote;

use crate::{map::MapFn, spin::RawSpinInputs};

#[test]
fn spin_parses_crate_tag_and_union_in_scope_and_new_union_and_map_in_scope() {
    let pound = syn::token::Pound::default();
    let input = quote::quote! {
        #pound example, U = f32 | u64, V, foo,
    };
    let spin_inputs: RawSpinInputs = parse_quote!(#input);
    let crate_tag = &spin_inputs.crate_tag;
    assert_eq!(&crate_tag.0.0.to_string(), "example");
    let u = spin_inputs.new_unions.get(0).unwrap();
    assert_eq!(&u.0.0.to_string(), "U");
    let fields: Vec<String> = u.1
        .iter()
        .map(|field| field.0.to_string())
        .collect();
    assert_eq!(fields, vec!["f32", "u64"]);
    
    let v = spin_inputs.unions_in_scope.get(0).unwrap();
    let ident = &v.0.0;
    assert_eq!(ident.to_string(), "V");
    
    let foo = spin_inputs.map_fns_in_scope.get(0).unwrap();
    let ident = &foo.0.0;
    assert_eq!(ident.to_string(), "foo");
}

#[allow(unused)]
#[test]
fn spin_gets_existing_map_from_db() {
    // connect to database
    // add function to database
    const DB_NAME: &str = "spin_gets_existing_map_from_db";
    const DB_PATH: &str = "target/spindle/db/";
    let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();

    // parse and insert a map into the db
    let map = quote::quote! {
        fn foo(x: u64) -> f32 {
            x as f32
        }
    };

    let tags: Vec<&str> = vec![];

    let map: MapFn = parse_quote!(#map);
    dbg!(&map);
    let db_map = db.get_or_insert_map(&map, &tags).unwrap();
    dbg!(&db_map);
    // after the parse and insert, we also write the trait module which also contains the uuid
    mod __foo {
        const __UUID: &str = "foo_uuid";
    }

    // we will also parse and add a union to the db
    let pound = syn::token::Pound::default();
    let spin_input = quote::quote! {
        #pound example, U = f32 | u64, foo
    };
    let spin_input: RawSpinInputs = parse_quote!(#spin_input);
    let u = spin_input.new_unions.get(0).unwrap();
    let f = spin_input.map_fns_in_scope.get(0).unwrap();
    dbg!(&spin_input);
    // todo!();
}

// #[test]
// fn spin_gets_crate_from_one_union_and_one_map() {
//     todo!("we're now adding maps to the spin syntax");
//     // // connect to database
//     // // add function to database
//     // const DB_NAME: &str = "spin_gets_crate_from_one_union_and_one_map";
//     // const DB_PATH: &str = "target/spindle/db/";
//     // let db = TypeDb::new(DB_NAME, DB_PATH).unwrap();

//     // // parse and insert a map into the db
//     // let map = quote::quote! {
//     //     fn foo(x: u64) -> f32 {
//     //         x as f32
//     //     }
//     // };
//     // let map: MapFn = parse_quote!(#map);
//     // dbg!(&map);
//     // let db_map = db.get_or_insert_map(&map).unwrap();
//     // dbg!(&db_map);

//     // // parse and insert a union into the db
//     // let union = quote::quote! {
//     //     U = f32 | u64
//     // };
//     // let spin_input: RawSpinInput = parse_quote!(#union);
//     // let db_union: DbUnion = db.get_or_insert_union(&spin_input).unwrap();
//     // dbg!(&db_union);

//     // // get the crate
//     // let db_crate = db.get_or_insert_crate_from_unions(vec![db_union]).unwrap();
//     // dbg!(&db_crate);
//     // assert_eq!(db_crate.unions.len(), 1);
// }
