use proc_macro2::{Ident, Span};
use quote::ToTokens;
use spindle_db::item_fn::DbItemFn;

use crate::map_fn::DevMapFn;

use super::{SpindleCrate, UnionInput};

impl ToTokens for SpindleCrate {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            home: _,
            maps,
            tag,
            unions,
        } = self;
        todo!();
        // let union_defs = unions.iter().map(|u| u.to_token_stream());
        // tokens.extend(quote::quote_spanned! { Span::mixed_site() =>
        //     #(#union_defs)*
        // });
        // assert_eq!(self.unions.len(), 1, "high-degree maps not yet supported");
        // let u_ident = match unions.get(0).unwrap() {
        //     UnionInput::New(ident, _) => &ident.0,
        //     UnionInput::InScope(ident) => &ident.0,
        // };
        // maps.iter().for_each(|map| {
        //     // for 1 union, impl on the DevSlice
        //     // for 2+ unions, impl on the tuple of DevSlices
        //     let mod_name = syn::Ident::new(&format!("__{}", map.ident), Span::call_site());
        //     // the trait is __UpperCamelCase
        //     use heck::ToUpperCamelCase;
        //     let trait_name = syn::Ident::new(
        //         &format!("__{}", map.ident.to_upper_camel_case()),
        //         Span::call_site(),
        //     );
        //     assert_eq!(map.in_outs.len(), 1, "high-degree maps not yet supported");
        //     let in_out = map.in_outs.get(0).unwrap();
        //     let input_ident: Ident = Ident::new(
        //         &in_out.input.as_ref().unwrap().ident.to_string(),
        //         Span::call_site(),
        //     );
        //     let output_ident: Ident = Ident::new(
        //         &in_out.output.as_ref().unwrap().ident.to_string(),
        //         Span::call_site(),
        //     );
        //     let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        //     let path = format!(
        //         "{target}/spindle/map/{tag}/target/nvptx64-nvidia-cuda/release/kernel.ptx"
        //     );
        //     let map_impl = quote::quote_spanned! { Span::mixed_site() =>
        //         unsafe impl #mod_name::#trait_name for spindle::DevSlice<#u_ident, #input_ident> {
        //             type U = #u_ident;
        //             type Return = spindle::DevSlice<#u_ident, #output_ident>;
        //             const PTX_PATH: &'static str = #path;

        //         }
        //     };
        //     tokens.extend(map_impl);
        // });
    }
}

impl ToTokens for UnionInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            UnionInput::New(ident, fields) => {
                let ident = &ident.0;
                let field_entries = fields.iter().enumerate().map(|(i, field)| {
                    let underscore_number = syn::Ident::new(&format!("_{i}"), Span::call_site());
                    let field_ident = &field.0;
                    quote::quote! {
                        #underscore_number: #field_ident,
                    }
                });
                tokens.extend(quote::quote! {
                    #[repr(C)]
                    pub union #ident {
                        #(#field_entries)*
                    }
                    unsafe impl spindle::__cudarc::DeviceRepr for #ident {}
                });
                fields.iter().for_each(|field| {
                    let field = &field.0;
                    tokens.extend(quote::quote! {
                        unsafe impl spindle::__union::RawConvert<#field> for #ident {}
                    })
                })
            }
            UnionInput::InScope(_) => {}
        }
    }
}

impl SpindleCrate {
    pub(crate) fn lib_rs(&self) -> proc_macro2::TokenStream {
        let Self {
            home: _,
            maps,
            tag: _,
            unions: _,
        } = self;

        let preamble = quote::quote! {
            #![no_std]
            #![feature(abi_ptx)]
            #![feature(stdsimd)]
            // #![feature(core_intrinsics)]

            use core::arch::nvptx::*;

            mod device;
            use device::*;

            #[panic_handler]
            fn my_panic(_: &core::panic::PanicInfo) -> ! {
                loop {}
            }
        };

        assert_eq!(self.unions.len(), 1, "high-degree maps not yet supported");
        let union = self.unions.get(0).unwrap();
        let union_ident = match union {
            crate::spin::UnionInput::New(ident, _) => &ident.0,
            crate::spin::UnionInput::InScope(ident) => &ident.0,
        };

        todo!();
        // let maps = maps.iter().map(|map| {
        //     let DbMap {
        //         uuid: _,
        //         ident,
        //         content: _,
        //         in_outs,
        //         range_type,
        //     } = map;
        //     assert_eq!(in_outs.len(), 1, "high-degree maps not yet supported");
        //     assert!(range_type.is_none(), "range types not yet supported");
        //     let map_ident = syn::Ident::new(
        //         ident,
        //         Span::call_site(),
        //     );
        //     let kernel_ident = syn::Ident::new(
        //         &format!("{}_kernel", ident),
        //         Span::call_site(),
        //     );
        //     quote::quote! {
        //         #[no_mangle]
        //         pub unsafe extern "ptx-kernel" fn #kernel_ident(slice: *mut #union_ident, size: i32) {
        //             // todo! try other thread geometry
        //             let thread_id: i32 = _thread_idx_x();
        //             let block_id: i32 = _block_idx_x();
        //             let block_dim: i32 = _block_dim_x();
        //             let grid_dim: i32 = _grid_dim_x();
                    
        //             let n_threads: i32 = block_dim * grid_dim;
        //             let thread_index: i32 =  thread_id + block_id * block_dim;

        //             let mut i: i32 = thread_index;
        //             while i < size {
        //                 let u: &mut #union_ident = &mut *slice.offset(i as isize);
        //                 u.#map_ident();
        //                 i = i.wrapping_add(n_threads);
        //             }
        //         }
        //     }
        // });
        // quote::quote_spanned! { Span::mixed_site() =>
        //     #preamble
        //     #(#maps)*
        // }
    }

    pub(crate) fn device_rs(&self) -> proc_macro2::TokenStream {
        let Self {
            home: _,
            maps,
            tag: _,
            unions,
        } = self;
        assert_eq!(self.unions.len(), 1, "high-degree maps not yet supported");
        let union = unions.get(0).unwrap();
        let union_ident = match union {
            crate::spin::UnionInput::New(ident, _) => &ident.0,
            crate::spin::UnionInput::InScope(ident) => &ident.0,
        };
        let pound: syn::Token![#] = Default::default();
        let device_maps = maps.iter().map(|map| {
            let map_fn: DevMapFn = syn::parse_str(&map.content).unwrap();
            map_fn
        });
        let methods = maps.iter().map(|map| {
            let map_ident = syn::Ident::new(&map.ident, Span::call_site());
            quote::quote! {
                pub(crate) unsafe fn #map_ident(&mut self) {
                    let input_ref = &*(self as *mut _ as *mut _);
                    let output = #map_ident(*input_ref);
                    let output_ptr: *mut _ = self as *mut _ as _;
                    *output_ptr = output;
                }
            }
        });
        quote::quote_spanned! { Span::mixed_site() =>
            #( #device_maps )*
            #pound[repr(C)]
            pub union #union_ident {
                _0: i32,
                _1: f64,
            }
            impl #union_ident {
                #( #methods )*
            }
        }
    }
}
