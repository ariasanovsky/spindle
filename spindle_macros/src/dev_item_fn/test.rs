use quote::ToTokens;
use spindle_db::TypeDb;
use syn::{ItemFn, Error};

use super::DevItemFn;

#[test]
fn device_item_fn_parse_prohibits_attributes() {
    let item_fn: ItemFn = syn::parse_quote! {
        #[inline]
        // const
        // async
        // unsafe
        // extern "C"
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let device_item_fn: Result<DevItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn device_item_fn_parse_prohibits_const() {
    let item_fn: ItemFn = syn::parse_quote! {
        // #[inline]
        const
        // async
        // unsafe
        // extern "C"
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let device_item_fn: Result<DevItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn device_item_fn_parse_prohibits_async() {
    let item_fn: ItemFn = syn::parse_quote! {
        // #[inline]
        // const
        async
        // unsafe
        // extern "C"
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let device_item_fn: Result<DevItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn device_item_fn_parse_prohibits_unsafe() {
    let item_fn: ItemFn = syn::parse_quote! {
        // #[inline]
        // const
        // async
        unsafe
        // extern "C"
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let device_item_fn: Result<DevItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn device_item_fn_parse_prohibits_abi() {
    let item_fn: ItemFn = syn::parse_quote! {
        // #[inline]
        // const
        // async
        // unsafe
        extern "C"
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let device_item_fn: Result<DevItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn device_item_fn_parse_prohibits_variadic() {
    let item_fn: ItemFn = syn::parse_quote! {
        // #[inline]
        // const
        // async
        // unsafe
        // extern "C"
        fn foo(x: i32, ...) -> f64 {
            x as f64
        }
    };
    let device_item_fn: Result<DevItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn parse_univariate_pure_dev_item_fn_and_compare_with_db_item_fn() {
    let device_item_fn: DevItemFn = syn::parse_quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let expected_token_stream = quote::quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    assert_eq!(device_item_fn.to_token_stream().to_string(), expected_token_stream.to_string());
    let db = TypeDb::open_empty_db_in_memory().unwrap();
    db.create_new_item_fn_table().unwrap();
    db.create_new_tag_table().unwrap();
    db.create_new_tagged_item_fn_table().unwrap();
    let tags = vec!["fizz", "buzz"];
    let db_item_fn = db.get_or_insert_item_fn(&device_item_fn, &tags).unwrap();
    let db_dev_item_fn: DevItemFn = db_item_fn.try_into().unwrap();
    assert_eq!(device_item_fn.to_token_stream().to_string(), db_dev_item_fn.to_token_stream().to_string());
}

