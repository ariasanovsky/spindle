use proc_macro2::Ident;

use crate::primitives::Primitive;

mod parse;

#[derive(Clone)]
pub struct _Union {
    ident: Ident,
    fields: Vec<Primitive>,
}
