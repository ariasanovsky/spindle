use std::path::PathBuf;

use proc_macro2::TokenStream;
use syn::{parse::Parse, parse::ParseStream, Ident, LitStr, Token};

use crate::{
    error::NaivelyTokenize,
    file_strings::{CARGO_TOML, CONFIG_TOML, RUST_TOOLCHAIN_TOML},
    map::{MapFn, MapFnStrings},
    snake_to_camel, TokenResult, case::{UpperCamelIdent, LowerSnakeIdent, PrimitiveIdent},
};

mod parse;
#[cfg(test)]
mod test;

pub(crate) enum RawSpinInput {
    OldUnion(UpperCamelIdent),
    NewUnion(UpperCamelIdent, Vec<PrimitiveIdent>),
}

impl RawSpinInput {
    pub(crate) fn ident(&self) -> &UpperCamelIdent {
        match self {
            Self::OldUnion(ident) => ident,
            Self::NewUnion(ident, _) => ident,
        }
    }

    pub(crate) fn fields(&self) -> Option<&Vec<PrimitiveIdent>> {
        match self {
            Self::OldUnion(_) => None,
            Self::NewUnion(_, fields) => Some(fields),
        }
    }
}

pub(crate) struct SpinInput {
    pub(crate) union_name: Ident,
    pub(crate) types: Vec<Ident>,
}

impl Parse for SpinInput {
    // todo! ?new grammar `U = p | q | r` or `V` comma separated
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let union_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let types: syn::punctuated::Punctuated<Ident, Token![,]> =
            input.parse_terminated(Ident::parse, Token![,])?;
        Ok(SpinInput {
            union_name,
            types: types.into_iter().collect(),
        })
    }
}

impl SpinInput {
    pub(crate) fn union(&self) -> TokenStream {
        let Self { union_name, types } = self;
        let union_fields = types.iter().enumerate().map(|(i, ty)| {
            let field_name = format!("_{i}");
            let field_name = syn::Ident::new(&field_name, proc_macro2::Span::call_site());
            quote::quote! { #field_name: #ty }
        });
        quote::quote! {
            #[repr(C)]
            pub union #union_name {
                #(#union_fields),*
            }
        }
    }

    pub(crate) fn impls(&self) -> TokenStream {
        let Self { union_name, types } = self;
        let impls = types.iter().map(|ty| {
            quote::quote! {
                unsafe impl spindle::spindle::RawConvert<#ty> for #union_name {}
            }
        });
        quote::quote! {
            unsafe impl cudarc::driver::DeviceRepr for #union_name {}
            #(#impls)*
        }
    }

    pub(crate) fn emit_map_kernels_and_return_spindle_impls(&self) -> TokenResult {
        // get all map strings from $PROJECT/target/spindle/map/*.json
        let map_dir = std::path::PathBuf::from("target/spindle/map/");
        let map_paths = std::fs::read_dir(map_dir)
            .map_err(NaivelyTokenize::naively_tokenize)?
            .map(|entry| entry.map_err(NaivelyTokenize::naively_tokenize))
            .collect::<Result<Vec<_>, _>>()?;
        let map_fn_strings: Vec<MapFnStrings> = map_paths
            .iter()
            .filter(|path| {
                path.path()
                    .extension()
                    .is_some_and(|path| path.to_str().is_some_and(|path| path.eq("json")))
            })
            .map(|path| {
                let map_fn_strings = std::fs::read_to_string(path.path())
                    .map_err(NaivelyTokenize::naively_tokenize)?;
                let map_fn_strings: MapFnStrings = serde_json::from_str(&map_fn_strings)
                    .map_err(NaivelyTokenize::naively_tokenize)?;
                Ok(map_fn_strings)
            })
            .collect::<Result<Vec<_>, TokenStream>>()?;
        // todo! this ignores MapAttrs
        let map_fns: Vec<MapFn> = map_fn_strings
            .iter()
            .map(|map_fn_strings| syn::parse_str::<MapFn>(&map_fn_strings.1).unwrap())
            .collect();
        /* with $HOME = $PROJECT/target/spindle/#union_name/
            [x] write Cargo.toml, rust-toolchain.toml, .cargo/config.toml
            [x] write src/foo.rs and src/bar.rs
            [x] write src/union.rs w/ union U { _0: i32, _1: f64, ... }
            [x] write the methods foo & bar for U
            [x] write src/lib.rs w/ fn foo_kernel, fn bar_kernel, etc.
            [x] compile the crate
            capture the ptx
            😓 (sloppy first drafts are fine here)
        */

        let crate_home = PathBuf::from("target/spindle/map/").join(self.union_name.to_string());
        let union_name = &self.union_name;
        let union = self.union();

        let trait_and_impl_maker = |map_fn: MapFn| {
            // todo! belongs w/ MapFn
            let fn_name = &map_fn.item_fn.sig.ident;
            let trimmed_name = fn_name.to_string().trim_start_matches('_').to_string();
            let fn_name = proc_macro2::Ident::new(&trimmed_name, proc_macro2::Span::call_site());

            let mod_name = format!("__{trimmed_name}");
            let mod_name = proc_macro2::Ident::new(&mod_name, proc_macro2::Span::call_site());
            let trait_name = snake_to_camel(&fn_name.to_string());
            let trait_name = proc_macro2::Ident::new(&trait_name, proc_macro2::Span::call_site());
            let input_type = match map_fn.item_fn.sig.inputs.first().unwrap() {
                syn::FnArg::Receiver(_) => todo!("so much todo"),
                syn::FnArg::Typed(pat_type) => &pat_type.ty,
            };
            let output_type = match &map_fn.item_fn.sig.output {
                syn::ReturnType::Default => todo!("so much todo"),
                syn::ReturnType::Type(_, t) => t,
            };

            let union_definition = self.union();

            let kernel_name: LitStr = LitStr::new(
                &format!("{trimmed_name}_kernel"),
                proc_macro2::Span::call_site(),
            );
            let union_name_lit: LitStr =
                LitStr::new(&union_name.to_string(), proc_macro2::Span::call_site());

            quote::quote! {
                mod #mod_name {
                    use cudarc::{
                        driver::{
                            CudaDevice, CudaFunction, CudaSlice, DeviceRepr, DeviceSlice, LaunchAsync, LaunchConfig,
                        },
                        nvrtc::Ptx,
                    };
                    use spindle::error::Error;
                    use std::{path::PathBuf, sync::Arc};

                    pub unsafe trait #trait_name
                    where
                        <Self as #trait_name>::U: DeviceRepr,
                        Self: Into<CudaSlice<Self::U>>,
                        CudaSlice<<Self as #trait_name>::U>: Into<<Self as #trait_name>::Return>,
                    {
                        type U;
                        type Return;
                        unsafe fn #fn_name(self) -> Result<Self::Return, Error> {
                            let mut slice: CudaSlice<Self::U> = self.into();
                            let n: usize = slice.len();
                            let device: Arc<CudaDevice> = slice.device();
                            let ptx_path: PathBuf = PathBuf::from("target/spindle/map/")
                                .join(#union_name_lit)
                                // .join("U")
                                .join("target")
                                .join("nvptx64-nvidia-cuda")
                                .join("release")
                                .join("kernel.ptx");
                            let ptx_contents: String = std::fs::read_to_string(&ptx_path)?;
                            let _res: () = device.load_ptx(
                                Ptx::from_src(ptx_contents),
                                "kernel",
                                &[#kernel_name],
                            )?;
                            let f: CudaFunction = device
                                .get_func("kernel", #kernel_name)
                                .ok_or(Error::FunctionNotFound)?;
                            let config: LaunchConfig = LaunchConfig::for_num_elems(n as u32); // todo! inspect cudarc fn
                            unsafe { f.launch(config, (&mut slice, n as i32)) }
                                .map(|()| slice.into())
                                .map_err(Into::into)
                        }
                    }

                    #union_definition

                    unsafe impl #trait_name for spindle::DevSpindle<#union_name, #input_type> {
                        type U = #union_name;
                        type Return = spindle::DevSpindle<#union_name, #output_type>;
                    }
                }
                use #mod_name::*;
            }
        };

        let map_fn_impls = map_fns
            .iter()
            .map(|map_fn| trait_and_impl_maker(map_fn.clone()));

        let map_fn_impls = quote::quote! {
            #(#map_fn_impls)*
        };

        dbg!("ahhhh");
        let ahhh = map_fn_impls.to_string();
        println!("{ahhh}");

        // todo! temporary safeguard: if the crate_home already exists
        if crate_home.exists() {
            // todo! this is a hack
            return Ok(map_fn_impls);
        }

        std::fs::create_dir_all(&crate_home).map_err(NaivelyTokenize::naively_tokenize)?;
        // copy the template toml files
        std::fs::write(crate_home.join("Cargo.toml"), CARGO_TOML)
            .map_err(NaivelyTokenize::naively_tokenize)?;
        std::fs::write(crate_home.join("rust-toolchain.toml"), RUST_TOOLCHAIN_TOML)
            .map_err(NaivelyTokenize::naively_tokenize)?;
        std::fs::create_dir_all(crate_home.join(".cargo"))
            .map_err(NaivelyTokenize::naively_tokenize)?;
        std::fs::write(crate_home.join(".cargo/config.toml"), CONFIG_TOML)
            .map_err(NaivelyTokenize::naively_tokenize)?;
        // src
        std::fs::create_dir_all(crate_home.join("src"))
            .map_err(NaivelyTokenize::naively_tokenize)?;
        // screw it, let's just dump all functions into union.rs for now
        let method_maker = |ident: Ident| {
            quote::quote! {
                impl #union_name {
                    pub(crate) unsafe fn #ident(&mut self) {
                        let input_ref = &*(self as *mut _ as *mut _);
                        let output = #ident(*input_ref);
                        let output_ptr: *mut _ = self as *mut _ as _;
                        *output_ptr = output;
                    }
                }
            }
        };

        let methods = map_fns.iter().map(|map_fn| {
            let ident = &map_fn.item_fn.sig.ident;
            method_maker(ident.clone())
        });

        let union_rs = quote::quote! {
            #union
            #(#map_fns)*
            #(#methods)*
        };
        std::fs::write(crate_home.join("src/device.rs"), union_rs.to_string())
            .map_err(NaivelyTokenize::naively_tokenize)?;

        let kernel_maker = |fn_name: Ident| {
            // let fn_name = &map_fn.0.sig.ident;
            let trimmed_name = fn_name.to_string().trim_start_matches('_').to_string();

            let kernel_name = proc_macro2::Ident::new(
                &format!("{trimmed_name}_kernel"),
                proc_macro2::Span::call_site(),
            );
            quote::quote! {
                    #[no_mangle]
                    pub unsafe extern "ptx-kernel" fn #kernel_name(slice: *mut #union_name, size: i32) {
                        let thread_id: i32 = _thread_idx_x();
                        let block_id: i32 = _block_idx_x();
                        let block_dim: i32 = _block_dim_x();
                        let grid_dim: i32 = _grid_dim_x();

                        let n_threads: i32 = block_dim * grid_dim;
                        let thread_index: i32 =  thread_id + block_id * block_dim;

                        let mut i: i32 = thread_index;
                        while i < size {
                            let u: &mut #union_name = &mut *slice.offset(i as isize);
                            u.#fn_name();
                            i = i.wrapping_add(n_threads);
                        }
                    }
            }
        };

        let kernels = map_fns.iter().map(|map_fn| {
            let ident = &map_fn.item_fn.sig.ident;
            kernel_maker(ident.clone())
        });

        let lib_rs = quote::quote! {
            #![no_std]
            #![feature(abi_ptx)]
            #![feature(stdsimd)]
            #![feature(core_intrinsics)]

            use core::arch::nvptx::*;

            mod device;
            use device::*;

            #[panic_handler]
            fn my_panic(_: &core::panic::PanicInfo) -> ! {
                loop {}
            }

            #(#kernels)*
        };

        std::fs::write(crate_home.join("src/lib.rs"), lib_rs.to_string())
            .map_err(NaivelyTokenize::naively_tokenize)?;

        let mut cmd = std::process::Command::new("cargo");
        cmd.args([
            "+nightly",
            "-Z",
            "unstable-options",
            "-C",
            &crate_home.to_string_lossy(),
            "build",
            "--release",
        ]);
        let output = cmd.output().map_err(NaivelyTokenize::naively_tokenize)?;
        println!(
            "stderr: {}\nstdout: {}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );

        Ok(map_fn_impls)
    }
}
