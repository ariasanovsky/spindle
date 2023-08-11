use proc_macro2::{TokenStream, Ident};
use quote::ToTokens;

use crate::{case::UpperCamelIdent, map::MapFn, snake_to_camel};

pub(crate) trait MapTokens {
    fn user_crate_declaration(&self) -> TokenStream;
    fn user_crate_trait(&self, uuid: &str) -> TokenStream;
    fn ptx_crate_method(&self, u: &UpperCamelIdent) -> TokenStream;
    fn ptx_crate_declaration(&self) -> TokenStream;
    fn ptx_crate_kernel(&self, u: &UpperCamelIdent) -> TokenStream;
}

// todo! actually, let's convert the DbMap back into a MapFn outside
impl MapTokens for MapFn {
    fn user_crate_declaration(&self) -> TokenStream {
        self.into_token_stream()
    }

    fn user_crate_trait(&self, uuid: &str) -> TokenStream {
        let dunder_mod_ident = Ident::new(format!("__{}", self.item_fn.sig.ident).as_str(), self.item_fn.sig.ident.span());
        dbg!(&dunder_mod_ident);
        let dunder_camel_trait_ident = Ident::new(format!("__{}", snake_to_camel(&self.item_fn.sig.ident.to_string())).as_str(), self.item_fn.sig.ident.span());
        dbg!(&dunder_camel_trait_ident);
        let method_ident = &self.item_fn.sig.ident;
        let kernel_name_string = format!("{}_kernel", self.item_fn.sig.ident);
        quote::quote! {
            mod #dunder_mod_ident {
                const __UUID: &str = #uuid;
                use spindle::__cudarc::{
                    CudaDevice as __CudaDevice,
                    CudaFunction as __CudaFunction,
                    CudaSlice as __CudaSlice,
                    DeviceRepr as __DeviceRepr,
                    LaunchConfig as __LaunchConfig,
                    Ptx as __Ptx,
                };
                unsafe trait #dunder_camel_trait_ident
                where
                    <Self as #dunder_camel_trait_ident>::U:
                        __DeviceRepr,
                    Self:
                        Into<__CudaSlice<<Self as #dunder_camel_trait_ident>::U>>,
                    __CudaSlice<<Self as #dunder_camel_trait_ident>::U>:
                        Into<<Self as #dunder_camel_trait_ident>::Return>,
                {
                    type U;
                    type Return;
                    const PTX_PATH: &'static str;
                    fn #method_ident(&self, n: u32) -> spindle::Result<Self::Return> {
                        let mut slice: __CudaSlice<Self::U> = self.into();
                        let device: std::sync::Arc<__CudaDevice> = slice.device();
                        let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
                        device.load_ptx(ptx, "kernels", &[#kernel_name_string])?;
                        let f: __CudaFunction =
                            device.get_function(#kernel_name_string)
                            .ok_or(spindle::Error::FunctionNotFound)?;
                        let config: __LaunchConfig = __LaunchConfig::for_num_elems(n as u32);
                        unsafe { f.launch(config, (&mut slice, n as i32)) }?;
                        Ok(slice.into())
                    }
                }
            }
        }
    }

    fn ptx_crate_method(&self, u: &UpperCamelIdent) -> TokenStream {
        let u = &u.0;
        let map_ident = &self.item_fn.sig.ident;
        quote::quote! {
            impl #u {
                pub(crate) unsafe fn #map_ident(&mut self) {
                    let input_ref = &*(self as *mut _ as *mut _);
                    let output = foo(*input_ref);
                    let output_ptr: *mut _ = self as *mut _ as _;
                    *output_ptr = output;
                }
            }
        }
    }

    fn ptx_crate_declaration(&self) -> TokenStream {
        self.to_token_stream()
    }

    fn ptx_crate_kernel(&self, u: &UpperCamelIdent) -> TokenStream {
        let u = &u.0;
        let map_ident = &self.item_fn.sig.ident;
        let map_kernel_ident = Ident::new(format!("{}_kernel", map_ident).as_str(), self.item_fn.sig.ident.span());
        quote::quote! {
            #[no_mangle]
            pub unsafe extern "ptx-kernel" fn #map_kernel_ident(slice: *mut #u, size: i32) {
                // todo! try other thread geometry
                let thread_id: i32 = _thread_idx_x();
                let block_id: i32 = _block_idx_x();
                let block_dim: i32 = _block_dim_x();
                let grid_dim: i32 = _grid_dim_x();
                
                let n_threads: i32 = block_dim * grid_dim;
                let thread_index: i32 =  thread_id + block_id * block_dim;

                let mut i: i32 = thread_index;
                while i < size {
                    let u: &mut #u = &mut *slice.offset(i as isize);
                    u.foo();
                    i = i.wrapping_add(n_threads);
                }
            }
        }
    }
}