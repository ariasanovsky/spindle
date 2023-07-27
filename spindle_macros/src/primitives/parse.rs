use syn::parse::Parse;

use super::Primitive;

impl Parse for Primitive {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        // TODO! < 0.2.0 check case
        let ident = crate::case::LowerSnakeIdent(ident);
        let field = Primitive::new_primitive(ident);
        // check if the sqlite database already has this field
        todo!()
    }
}