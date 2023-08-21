use syn::{ItemFn, Error};

use super::DeviceItemFn;

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
    let device_item_fn: Result<DeviceItemFn, Error> = item_fn.try_into();
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
    let device_item_fn: Result<DeviceItemFn, Error> = item_fn.try_into();
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
    let device_item_fn: Result<DeviceItemFn, Error> = item_fn.try_into();
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
    let device_item_fn: Result<DeviceItemFn, Error> = item_fn.try_into();
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
    let device_item_fn: Result<DeviceItemFn, Error> = item_fn.try_into();
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
    let device_item_fn: Result<DeviceItemFn, Error> = item_fn.try_into();
    assert!(device_item_fn.is_err());
}

#[test]
fn device_item_fn_parse_univariate_pure_fn() {
    let item_fn: ItemFn = syn::parse_quote! {
        fn foo(x: i32) -> f64 {
            x as f64
        }
    };
    let device_item_fn: DeviceItemFn = item_fn.try_into().unwrap();
}

