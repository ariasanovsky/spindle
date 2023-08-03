use syn::parse::Parse;

use crate::case::LowerSnakeIdent;

use super::_Primitive;

impl Parse for _Primitive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // first, parse as an ident
        let ident = input.parse::<syn::Ident>()?;
        // then, convert to a LowerSnakeIdent
        let lower_ident = LowerSnakeIdent::try_from(ident).map_err(|_| {
            syn::Error::new(input.span(), "expected lower_snake_case")
        })?;
        todo!()
        // then, update the database
        // let db = crate::db::TypeDb::connect().map_err(|err| {
        //     syn::Error::new(input.span(), err)
        // })?;
        // db.add_primitive_if_not_exists(lower_ident)
        // .map_err(|err| {
        //     syn::Error::new(input.span(), err)
        // })
    }
}