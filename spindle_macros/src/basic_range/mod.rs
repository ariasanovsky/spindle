use std::path::PathBuf;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{
    error::{command_output_result, NaivelyTokenize},
    snake_to_camel, BasicRangeAttrs, BasicRangeFn, RangeSpindle, TokenResult,
};

pub(crate) mod parse;

static RANGE_FILES: &[(&str, &str, &str)] = &[
    ("Cargo.toml", "", super::file_strings::CARGO_TOML),
    (
        "rust-toolchain.toml",
        "",
        super::file_strings::RUST_TOOLCHAIN_TOML,
    ),
    ("config.toml", ".cargo", super::file_strings::CONFIG_TOML),
    ("device.rs", "src", ""),
    ("lib.rs", "src", super::file_strings::LIB_RS),
    ("kernel.ptx", "target/nvptx64-nvidia-cuda/release", ""),
];

impl RangeSpindle {
    fn generate(name: &str, device: &BasicRangeFn) -> Result<Self, TokenStream> {
        let spindle = PathBuf::from(KERNELS).join(name).with_extension("json");
        let new_device = device.clone().into_token_stream().to_string();
        let spindle = if spindle.exists() {
            let spindle =
                std::fs::read_to_string(spindle).map_err(NaivelyTokenize::naively_tokenize)?;
            let mut spindle: RangeSpindle =
                serde_json::from_str(&spindle).map_err(NaivelyTokenize::naively_tokenize)?;
            spindle.update_device(new_device)?;
            spindle
        } else {
            Self {
                home: String::from(KERNELS),
                name: name.into(),
                populated: false,
                compiled: false,
                device: Some(new_device),
                msg: None,
                kernel: None,
            }
        };
        let path = PathBuf::from(KERNELS).join(name);
        std::fs::create_dir_all(&path).map_err(NaivelyTokenize::naively_tokenize)?;
        std::fs::create_dir_all(path.join(".cargo")).map_err(NaivelyTokenize::naively_tokenize)?;
        std::fs::create_dir_all(path.join("src")).map_err(NaivelyTokenize::naively_tokenize)?;
        if !spindle.populated {
            for (name, dir, contents) in RANGE_FILES {
                if contents.ne(&"") {
                    std::fs::write(path.join(dir).join(name), contents)
                        .map_err(NaivelyTokenize::naively_tokenize)?;
                }
            }
            let device = device.into_token_stream().to_string();
            std::fs::write(path.join("src/device.rs"), device)
                .map_err(NaivelyTokenize::naively_tokenize)?;
        }

        Ok(spindle)
    }

    fn remove_files(&mut self) -> Result<(), TokenStream> {
        let path = PathBuf::from(&self.home).join(&self.name);
        for (file, dir, _) in RANGE_FILES {
            let path = path.join(dir).join(file);
            if path.exists() {
                std::fs::remove_file(path).map_err(NaivelyTokenize::naively_tokenize)?;
            }
        }
        let Self {
            home: _home,
            name: _name,
            populated,
            compiled,
            device,
            msg,
            kernel,
        } = self;

        *populated = false;
        *compiled = false;
        *device = None;
        *msg = None;
        *kernel = None;
        self.write()?;
        Ok(())
    }

    fn update_device(&mut self, new_device: String) -> Result<(), TokenStream> {
        let Self {
            home: _,
            name: _,
            populated,
            compiled,
            device,
            msg,
            kernel,
        } = self;

        if device.as_ref().is_some_and(|device| new_device.eq(device)) {
            return Ok(());
        }

        *compiled = false;
        *populated = false;
        *device = Some(new_device);
        *msg = None;
        *kernel = None;
        self.remove_files()?;
        self.write()
    }

    fn write(&self) -> Result<(), TokenStream> {
        let json =
            serde_json::to_string_pretty(&self).map_err(NaivelyTokenize::naively_tokenize)?;
        let crate_json = PathBuf::from(&self.home)
            .join(&self.name)
            .with_extension("json");
        std::fs::write(crate_json, json).map_err(NaivelyTokenize::naively_tokenize)
    }

    fn compile(&mut self) -> Result<String, TokenStream> {
        let mut cmd = std::process::Command::new("cargo");
        let home = format!("{}/{}", self.home, self.name);
        cmd.args([
            "+nightly",
            "-Z",
            "unstable-options",
            "-C",
            &home,
            "build",
            "--release",
        ]);
        let output = cmd.output().map_err(NaivelyTokenize::naively_tokenize)?;
        match command_output_result(output) {
            Ok(output) => {
                let Self {
                    home,
                    name,
                    populated: _,
                    compiled,
                    device: _,
                    msg,
                    kernel,
                } = self;
                *compiled = true;
                *msg = Some(output.clone());
                let _kernel = PathBuf::from(&home)
                    .join(name)
                    .join("target")
                    .join("nvptx64-nvidia-cuda")
                    .join("release")
                    .join("kernel.ptx");
                *kernel = Some(
                    std::fs::read_to_string(_kernel).map_err(NaivelyTokenize::naively_tokenize)?,
                );
                self.write()?;
                Ok(output)
            }
            Err(err) => {
                let Self {
                    home: _,
                    name: _,
                    populated: _,
                    compiled,
                    device: _,
                    msg,
                    kernel,
                } = self;
                *compiled = false;
                *msg = Some(err.to_string());
                *kernel = None;
                self.write()?;
                Err(err.naively_tokenize())
            }
        }
    }
}

static KERNELS: &str = "target/spindle/";
// static RANGE_KERNEL: &'static str = include_str!("range/src/lib.rs");
// static RANGE_CARGO_TOML: &'static str = include_str!("range/Cargo.toml");

impl BasicRangeFn {
    fn name(&self) -> String {
        self.0.sig.ident.to_string()
    }

    fn rename(&mut self, name: &str) {
        self.0.sig.ident = syn::Ident::new(name, self.0.sig.ident.span());
    }

    fn make_visible(&mut self) {
        self.0.vis = syn::Visibility::Public(Default::default());
    }
}

pub(crate) fn emit_range_kernel(_attr: BasicRangeAttrs, item: BasicRangeFn) -> TokenResult {
    let name = item.name();
    let mut device = item.clone();
    device.make_visible();
    device.rename("device");
    let mut spindle = RangeSpindle::generate(&name, &device)?;
    const WARNING: &str = "\
        #![no_std] \
        #![feature(abi_ptx)] \
        #![feature(stdsimd)] \
        #![feature(core_intrinsics)] \
        core::arch::nvptx::*; \
    ";
    const COLOR: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";
    println!("{COLOR}{name} uses {}{}", WARNING, RESET);
    let output = spindle.compile()?;
    println!("{}", output.trim_end());

    let name = &item.0.sig.ident;
    let launch_name = syn::Ident::new(&format!("_{name}"), item.0.sig.ident.span());

    let input_type = match item.0.sig.inputs.first().unwrap() {
        syn::FnArg::Receiver(_) => todo!("Range fn is not a method"),
        syn::FnArg::Typed(p) => &p.ty,
    };

    let return_type = match &device.0.sig.output {
        syn::ReturnType::Default => unreachable!("RangeFn has a return type"),
        syn::ReturnType::Type(_, return_type) => return_type,
    };

    let trait_name = syn::Ident::new(
        &format!("_{}", snake_to_camel(&item.name())),
        item.0.sig.ident.span(),
    );

    let range_trait = quote::quote! {
        pub(super) trait #trait_name {
            type Returns;
            unsafe fn #name (&self) -> Result<Self::Returns, spindle::range::Error>;
        }
    };

    let ptx_path = syn::LitStr::new(
        &format!(
            "target/spindle/{}/target/nvptx64-nvidia-cuda/release/kernel.ptx",
            name
        ),
        name.span(),
    );

    let little_n = quote::quote! { n };
    let big_n = quote::quote! { N };

    let _host_alloc = |n: TokenStream| {
        quote::quote! {
            use cudarc::driver::safe::CudaSlice;
            use std::alloc::{alloc, dealloc};
            let mut out_host_ptr: *mut u8 = alloc(layout.clone());
            let out_host_vec: Vec<_> = if out_host_ptr.is_null() {
                dealloc(out_host_ptr, layout);
                return Err(Error::AllocationFailed);
            } else {
                Vec::from_raw_parts(out_host_ptr as *mut #return_type, #n, #n)
            };
            let mut out_dev: CudaSlice<_> = dev.htod_copy(out_host_vec)?;
        }
    };

    let dev_alloc = |n: TokenStream| {
        quote::quote! {
            use cudarc::driver::safe::CudaSlice;
            let mut out_dev: CudaSlice<_> = dev.alloc(#n)?;
        }
    };

    let launch_kernel = |n: TokenStream| {
        let alloc = dev_alloc(n.clone()); // _host_alloc(n.clone());
        quote::quote! {
            use spindle::range::Error;
            use cudarc::{driver::{CudaDevice, CudaFunction, DriverError, LaunchAsync, LaunchConfig}, nvrtc::Ptx};
            use core::alloc::Layout;
            use std::sync::Arc;

            let layout: Layout = Layout::array::<#return_type>(#n)?;
            let dev: Arc<CudaDevice> = CudaDevice::new(0)?;
            dev.load_ptx(
                Ptx::from_file(#ptx_path),
                "kernel",
                &["kernel"]
            )?;
            let f: CudaFunction = dev.get_func("kernel", "kernel").ok_or(Error::KernelNotFound)?;
            #alloc
            // todo! for_num_elems?
            let config: LaunchConfig = LaunchConfig::for_num_elems(#n as u32);
            f.launch(config, (&mut out_dev, #n as i32))?;
            let out_host_2: Vec<#return_type> = dev.sync_reclaim(out_dev)?;
        }
    };

    let launch_little_n = launch_kernel(little_n);
    let launch_big_n = launch_kernel(big_n);

    let int_impl = quote::quote! {
        impl #trait_name for #input_type {
            type Returns = Vec<#return_type>;
            unsafe fn #name (&self) -> Result<Self::Returns, spindle::range::Error> {
                let n: usize = *self as _; //todo! does `as` branch?
                #launch_little_n
                Ok(out_host_2)
            }
        }
    };
    let launcher = quote::quote! {
        pub(super) unsafe fn #launch_name <const N: usize>() -> Result<Box<[ #return_type ; N ]>, spindle::range::Error> {
            #launch_big_n
            out_host_2.try_into().map_err(|_| Error::LengthMismatch)

        }
    };

    let mod_name = syn::Ident::new(&format!("__{}", item.name()), item.0.sig.ident.span());

    Ok(quote::quote! {
        #item
        mod #mod_name {
            #range_trait
            #int_impl
            #launcher
        }
        use #mod_name::*;
    })
}

// gone, but not forgotten
// let out_host = unsafe { Box::from_raw(out_host as *mut [#return_type]) };
// dev.synchronize().unwrap();
// const LAYOUT: core::alloc::Layout = core::alloc::Layout::new::< [#return_type; N] >();
// let host_output: [#return_type; N] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
// let mut dev_output = dev.htod_copy(host_output.into()).unwrap();
// let mut dev_output: cudarc::driver::CudaSlice< #return_type > =
//     unsafe { dev.alloc(N) }.unwrap();
