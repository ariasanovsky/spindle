use syn::parse_quote;

use super::{RawSpinInput, RawSpinInputs};

#[test]
fn spin_parses_a_new_union_of_primitives() {
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
fn spin_parses_an_old_union() {
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
fn spin_parses_an_old_union_and_a_new_union_of_primitives() {
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