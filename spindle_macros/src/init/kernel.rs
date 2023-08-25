use proc_macro2::{Ident, Span};
use quote::ToTokens;

use crate::dev_item_fn::{DevFnIdent, DevReturnType, DevFnArg};

use super::{DevInitFn, DevInitSignature};

impl DevInitFn {
    pub fn kernel(&self, union_idents: &[Ident]) -> syn::Result<proc_macro2::TokenStream> {
        let Self { vis: _, sig, block: _ } = self;
        let DevInitSignature { fn_token: _, ident, paren_token: _, input, comma: _, output } = sig;
        let DevFnIdent(init_ident) = ident;
        let DevFnArg { pat: _, colon_token: _, ty: input_type } = input;
        use DevReturnType::*;
        match output {
            Default => unreachable!("DevInitFn::kernel should only be called on DevInitFn with a return type"),
            Type(_arrow, _output_type) => {
                match union_idents {
                    [u] => {
                        let init_kernel_ident = Ident::new(&format!("{init_ident}_init_kernel"), Span::mixed_site());
                        let init_method_ident = Ident::new(&format!("{init_ident}_init"), Span::mixed_site());
                        Ok(quote::quote_spanned! { Span::mixed_site() =>
                            #[no_mangle]
                            pub unsafe extern "ptx-kernel" fn #init_kernel_ident(slice: *mut #u, size: i32) {
                                let thread_id: i32 = _thread_idx_x();
                                let block_id: i32 = _block_idx_x();
                                let block_dim: i32 = _block_dim_x();
                                let grid_dim: i32 = _grid_dim_x();

                                let n_threads: i32 = block_dim * grid_dim;
                                let thread_index: i32 =  thread_id + block_id * block_dim;

                                let mut i: i32 = thread_index;
                                while i < size {
                                    let u: &mut #u = &mut *slice.offset(i as isize);
                                    u.#init_method_ident(i as #input_type);
                                    i = i.wrapping_add(n_threads);
                                }
                            }
                        })
                    },
                    _ => Err(syn::Error::new_spanned(init_ident, "expected exactly one union ident")),
                }
            },
            Tuple(_arrow, _tup) => todo!("tuple return types not yet supported"),
        }
    }

    pub fn device_item_fn(&self) -> proc_macro2::TokenStream {
        self.to_token_stream()
    }

    pub fn device_method(&self, union_idents: &[Ident]) -> syn::Result<proc_macro2::TokenStream> {
        let Self { vis: _, sig, block: _ } = self;
        let DevInitSignature { fn_token: _, ident, paren_token: _, input, comma: _, output } = sig;
        let DevFnIdent(init_ident) = ident;
        let DevFnArg { pat: _, colon_token: _, ty: input_type } = input;
        use DevReturnType::*;
        match output {
            Default => unreachable!("DevInitFn::kernel should only be called on DevInitFn with a return type"),
            Type(_arrow, _output_type) => {
                match union_idents {
                    [_] => {
                        let init_method_ident = Ident::new(&format!("{init_ident}_init"), Span::mixed_site());
                        Ok(quote::quote_spanned! { Span::mixed_site() =>
                            pub(crate) unsafe fn #init_method_ident(&mut self, i: #input_type) {
                                let init_value = #init_ident(i);
                                let output_ptr: *mut _ = self as *mut _ as _;
                                *output_ptr = init_value;
                            }
                        })
                    },
                    _ => Err(syn::Error::new_spanned(init_ident, "expected exactly one union ident")),
                }
            },
            Tuple(_arrow, _tup) => todo!("tuple return types not yet supported"),
        }
    }
}