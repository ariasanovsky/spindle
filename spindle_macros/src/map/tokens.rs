use proc_macro2::{TokenStream, Ident};
use quote::ToTokens;
use spindle_db::map::DbMap;

use crate::{case::UpperCamelIdent, map::MapFn, snake_to_camel};

pub(crate) trait MapTokens {
    fn user_crate_declaration(&self) -> TokenStream;
    fn user_crate_trait(&self) -> TokenStream;
    fn ptx_crate_method(&self, u: UpperCamelIdent) -> TokenStream;
    fn ptx_crate_declaration(&self) -> TokenStream;
    fn ptx_crate_kernel(&self) -> TokenStream;
}

// todo! actually, let's convert the DbMap back into a MapFn outside
impl MapTokens for MapFn {
    fn user_crate_declaration(&self) -> TokenStream {
        self.into_token_stream()
    }

    fn user_crate_trait(&self) -> TokenStream {
        let dunder_mod_name = Ident::new(format!("__{}", self.item_fn.sig.ident).as_str(), self.item_fn.sig.ident.span());
        dbg!(&dunder_mod_name);
        let dunder_camel_trait_name = Ident::new(format!("__{}", snake_to_camel(&self.item_fn.sig.ident.to_string())).as_str(), self.item_fn.sig.ident.span());
        dbg!(&dunder_camel_trait_name);
        quote::quote! {
            mod #dunder_mod_name {
                
            }
        }
    }

    fn ptx_crate_method(&self, u: UpperCamelIdent) -> TokenStream {
        dbg!(&u);
        todo!()
    }

    fn ptx_crate_declaration(&self) -> TokenStream {
        todo!()
    }

    fn ptx_crate_kernel(&self) -> TokenStream {
        todo!()
    }
}