use proc_macro2::{TokenStream, Ident, Span};
use spindle_db::union::DbUnion;

pub(crate) trait UnionTokens {
    fn declaration(&self) -> TokenStream;
    fn impl_uuid(&self) -> TokenStream;
    fn impl_device_repr(&self) -> TokenStream;
    fn impl_raw_converts(&self) -> Vec<TokenStream>;
}

impl UnionTokens for DbUnion {
    fn declaration(&self) -> TokenStream {
        let DbUnion { uuid: _uuid, ident, fields } = self;
        let fields = fields.iter().enumerate().map(|(i, field)| {
            // e.g., _0: f32, _1: u64, ...
            let field: Ident = Ident::new(&field.ident, Span::call_site());
            // since we have no span, we use ...
            let underscored_number: Ident = Ident::new(&format!("_{i}"), Span::call_site());
            quote::quote! { #underscored_number: #field, }
        });
        let ident: Ident = Ident::new(ident, Span::call_site());
        quote::quote! {
            #[repr(C)]
            union #ident {
                #(#fields)*
            }
        }
    }

    fn impl_uuid(&self) -> TokenStream {
        let DbUnion { uuid, ident, fields: _fields } = self;
        let ident: Ident = Ident::new(ident, Span::call_site());
        quote::quote! {
            unsafe impl spindle::__db::DbUuid for #ident {
                const __UUID: &'static str = #uuid;
            }
        }
    }

    fn impl_device_repr(&self) -> TokenStream {
        let DbUnion { uuid: _uuid, ident, fields: _fields } = self;
        let ident: Ident = Ident::new(ident, Span::call_site());
        quote::quote! {
            unsafe impl spindle::__cudarc::DeviceRepr for #ident {}
        }
    }

    fn impl_raw_converts(&self) -> Vec<TokenStream> {
        let DbUnion { uuid: _uuid, ident, fields } = self;
        let ident: Ident = Ident::new(ident, Span::call_site());
        fields.iter().map(|field| {
            let field: Ident = Ident::new(&field.ident, Span::call_site());
            quote::quote! {
                unsafe impl spindle::__union::RawConvert<#field> for #ident {}
            }
        }).collect()
    }
}