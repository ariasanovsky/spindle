use proc_macro2::{Span, Ident};

use super::DevInitFn;

impl DevInitFn {
    pub fn launch_trait(&self) -> proc_macro2::TokenStream {
        let ident = self.ident();
        let launch_trait = self.launch_trait_ident();
        let launch_mod = self.launch_mod_ident();
        quote::quote_spanned! { Span::mixed_site() =>
            mod #launch_mod {
                pub unsafe trait #launch_trait {
                    type Return;
                    const PTX_PATH: &'static str;
                    fn #ident(&self, size: i32) -> spindle::Result<Self::Return>;        
                }
            }
            pub use #launch_mod::#launch_trait;
        }
    }

    pub fn launch_impl(&self, tag: &str, union_idents: &[proc_macro2::Ident]) -> syn::Result<proc_macro2::TokenStream> {
        let ident = self.ident();
        let launch_trait = self.launch_trait_ident();
        let launch_mod = self.launch_mod_ident();
        // let input = self.input_type();
        let output = self.output();
        let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        let ptx_path = format!("{target_dir}/spindle/map/{tag}/target/nvptx64-nvidia-cuda/target/release/kernel.ptx");
        use crate::dev_item_fn::DevReturnType::*;
        match (output, union_idents) {
            (Default, _) => unreachable!("DevInitFn::launch_impl should only be called on DevInitFn with a return type"),
            (Type(_arrow, output_type), [u]) => {
                Ok(quote::quote_spanned! { Span::mixed_site() =>
                    unsafe impl #launch_mod::#launch_trait for std::ops::RangeFrom<u64> {
                        type Return = spindle::DevSlice<#u, #output_type>;
                        const PTX_PATH: &'static str = #ptx_path;
                        fn #ident(&self, size: i32) -> spindle::Result<Self::Return> {
                            use spindle::__cudarc::{
                                CudaDevice as __CudaDevice,
                                CudaFunction as __CudaFunction,
                                CudaSlice as __CudaSlice,
                                // DeviceRepr as __DeviceRepr,
                                LaunchAsync as __LaunchAsync,
                                LaunchConfig as __LaunchConfig,
                                Ptx as __Ptx,
                            };
                            let device: std::sync::Arc<__CudaDevice> = __CudaDevice::new(0)?;
                            let mut slice: __CudaSlice<_> = unsafe { device.alloc(size as usize) }?;
                            let ptx: __Ptx = __Ptx::from_file(Self::PTX_PATH);
                            device.load_ptx(ptx, "kernels", &["square_over_two_kernel"])?;
                            let f: __CudaFunction = device.get_func(
                                "kernels",
                                "square_over_two_kernel"
                            )
                            .ok_or(spindle::error::function_not_found(
                                Self::PTX_PATH,
                                "square_over_two_kernel"
                            ))?;
                            let config: __LaunchConfig = __LaunchConfig::for_num_elems(size as u32);
                            unsafe { f.launch(config, (&mut slice, size)) }?;
                            Ok(spindle::DevSlice::from(slice))
                        }
                    }
                })
            }
            (Type(_arrow, output_type), _) => unreachable!("DevInitFn::launch_impl should only be called when the number of unions equals the number of return types"),
            (Tuple(_, _), _) => todo!("DevInitFn::launch_impl for tuple return types (unreachable when lengths are not equal), else we gotta align pointers and stuff"),
        }
    }

    fn launch_trait_ident(&self) -> Ident {
        use heck::ToUpperCamelCase;
        let ident = self.ident().to_string();
        let ident = format!("__{}", ident.to_upper_camel_case());
        Ident::new(&ident, Span::mixed_site())
    }

    fn launch_mod_ident(&self) -> Ident {
        let ident = self.ident().to_string();
        let ident = format!("__{ident}");
        Ident::new(&ident, Span::mixed_site())
    }
}