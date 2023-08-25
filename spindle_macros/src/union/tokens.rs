use proc_macro2::{TokenStream, Span};
use quote::ToTokens;

use crate::case::{PrimitiveIdent, UpperCamelIdent};

use super::NewUnion;

pub(crate) trait UnionTokens {
    fn declaration(&self) -> TokenStream;
    fn impl_uuid(&self) -> TokenStream;
    fn impl_device_repr(&self) -> TokenStream;
    fn impl_raw_converts(&self) -> Vec<TokenStream>;
}

impl ToTokens for NewUnion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let NewUnion { ident, primitives } = self;
        let UpperCamelIdent(ident) = ident;
        let fields = primitives.iter().enumerate().map(|(i, primitive)| {
            let PrimitiveIdent(primitive) = primitive;
            let underscored_number = syn::Ident::new(&format!("_{}", i), Span::call_site());
            quote::quote! {
                #underscored_number: #primitive,
            }
        });
        let new_tokens = quote::quote_spanned! { Span::mixed_site() => 
            #[repr(C)]
            pub union #ident {
                #(#fields)*
            }
        };
        tokens.extend(new_tokens);
    }
}

// impl UnionTokens for DbUnion {
//     fn declaration(&self) -> TokenStream {
//         let DbUnion {
//             uuid: _uuid,
//             ident,
//             fields,
//         } = self;
//         let fields = fields.iter().enumerate().map(|(i, field)| {
//             // e.g., _0: f32, _1: u64, ...
//             let field: Ident = Ident::new(&field.ident, Span::call_site());
//             // since we have no span, we use ...
//             let underscored_number: Ident = Ident::new(&format!("_{i}"), Span::call_site());
//             quote::quote! { #underscored_number: #field, }
//         });
//         let ident: Ident = Ident::new(ident, Span::call_site());
//         quote::quote! {
//             #[repr(C)]
//             union #ident {
//                 #(#fields)*
//             }
//         }
//     }

//     fn impl_uuid(&self) -> TokenStream {
//         let DbUnion {
//             uuid,
//             ident,
//             fields: _fields,
//         } = self;
//         let ident: Ident = Ident::new(ident, Span::call_site());
//         quote::quote! {
//             unsafe impl spindle::__db::DbUuid for #ident {
//                 const __UUID: &'static str = #uuid;
//             }
//         }
//     }

//     fn impl_device_repr(&self) -> TokenStream {
//         let DbUnion {
//             uuid: _uuid,
//             ident,
//             fields: _fields,
//         } = self;
//         let ident: Ident = Ident::new(ident, Span::call_site());
//         quote::quote! {
//             unsafe impl spindle::__cudarc::DeviceRepr for #ident {}
//         }
//     }

//     fn impl_raw_converts(&self) -> Vec<TokenStream> {
//         let DbUnion {
//             uuid: _uuid,
//             ident,
//             fields,
//         } = self;
//         let ident: Ident = Ident::new(ident, Span::call_site());
//         fields
//             .iter()
//             .map(|field| {
//                 let field: Ident = Ident::new(&field.ident, Span::call_site());
//                 quote::quote! {
//                     unsafe impl spindle::__union::RawConvert<#field> for #ident {}
//                 }
//             })
//             .collect()
//     }
// }
