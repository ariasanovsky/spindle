use proc_macro2::TokenStream;
use syn::{parse::Parse, parse::ParseStream, Ident, Token};

pub(crate) struct SpinInput {
    pub(crate) union_name: Ident,
    pub(crate) types: Vec<Ident>,
}

impl Parse for SpinInput {
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
            let field_name = format!("_{}", i);
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
}
