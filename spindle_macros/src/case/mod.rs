use proc_macro2::Ident;

mod parse;

#[derive(Debug, Clone)]
pub struct LowerSnakeIdent(pub Ident);

#[derive(Debug, Clone)]
pub struct UpperCamelIdent(pub Ident);

#[derive(Debug, Clone)]
pub struct PrimitiveIdent(pub Ident);

// todo! legacy cruft
#[derive(Debug)]
pub enum Error {
    _NotLowerSnake(Ident),
    _NotUpperCamel(Ident),
}

// todo! legacy cruft
impl TryFrom<Ident> for LowerSnakeIdent {
    type Error = Error;

    fn try_from(_value: Ident) -> Result<Self, Self::Error> {
        todo!()
    }
}

// todo! legacy cruft
// impl TryFrom<Ident> for UpperCamelIdent {
//     type Error = Error;

//     fn try_from(value: Ident) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }
