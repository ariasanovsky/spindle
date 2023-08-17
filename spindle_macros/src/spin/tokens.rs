use proc_macro2::Span;
use quote::ToTokens;
use spindle_db::map::DbMap;

use crate::map::MapFn;

use super::SpindleCrate;

impl ToTokens for SpindleCrate {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            home: _,
            maps,
            tag: _,
            unions,
        } = self;
        todo!("e")
    }
}

impl SpindleCrate {
    pub(crate) fn lib_rs(&self) -> proc_macro2::TokenStream {
        let Self {
            home: _,
            maps,
            tag: _,
            unions,
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
        
        let maps = maps.iter().map(|map| {
            let DbMap {
                uuid: _,
                ident,
                content: _,
                in_outs,
            } = map;
            assert_eq!(in_outs.len(), 1, "high-degree maps not yet supported");
            let map_ident = syn::Ident::new(
                ident,
                Span::call_site(),
            );
            let kernel_ident = syn::Ident::new(
                &format!("{}_kernel", ident),
                Span::call_site(),
            );
            quote::quote! {
                #[no_mangle]
                pub unsafe extern "ptx-kernel" fn #kernel_ident(slice: *mut #union_ident, size: i32) {
                    // todo! try other thread geometry
                    let thread_id: i32 = _thread_idx_x();
                    let block_id: i32 = _block_idx_x();
                    let block_dim: i32 = _block_dim_x();
                    let grid_dim: i32 = _grid_dim_x();
                    
                    let n_threads: i32 = block_dim * grid_dim;
                    let thread_index: i32 =  thread_id + block_id * block_dim;

                    let mut i: i32 = thread_index;
                    while i < size {
                        let u: &mut #union_ident = &mut *slice.offset(i as isize);
                        u.#map_ident();
                        i = i.wrapping_add(n_threads);
                    }
                }
            }
        });
        quote::quote_spanned! { Span::mixed_site() =>
            #preamble
            #(#maps)*
        }
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
            let map_fn: MapFn = syn::parse_str(&map.content).unwrap();
            map_fn
        });
        let methods = maps.iter().map(|map| {
            let map_ident = syn::Ident::new(
                &map.ident,
                Span::call_site(),
            );
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