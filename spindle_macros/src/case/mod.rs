use proc_macro2::Ident;

#[derive(Debug, Clone)]
pub struct LowerSnakeIdent(pub Ident);

#[derive(Debug, Clone)]
pub struct UpperCamelIdent(pub Ident);

#[derive(Debug)]
pub enum Error {
    _NotLowerSnake(Ident),
    _NotUpperCamel(Ident),
}

impl TryFrom<Ident> for LowerSnakeIdent {
    type Error = Error;

    fn try_from(_value: Ident) -> Result<Self, Self::Error> {
        todo!()
    }
}

// impl TryFrom<Ident> for UpperCamelIdent {
//     type Error = Error;

//     fn try_from(value: Ident) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }
