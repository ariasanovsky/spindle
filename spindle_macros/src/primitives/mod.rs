// use proc_macro2::{Ident, Span};
use crate::{case::LowerSnakeIdent, regulate::ident};

// pub mod db;
pub mod parse;

// todo! ?refactor to use `PrimitiveIdent` instead of this useless struct
#[derive(Debug, Clone)]
pub struct _Primitive {
    pub ident: LowerSnakeIdent,
}

impl PartialEq for _Primitive {
    fn eq(&self, other: &Self) -> bool {
        let Self { ident } = self;
        let Self { ident: other_ident } = other;
        ident.0.to_string() == other_ident.0.to_string()
    }
}

impl _Primitive {
    pub fn is_integer(&self) -> bool {
        let Self { ident } = self;
        let LowerSnakeIdent(ident) = ident;
        const INTEGERS: &[&str] = &[
            "i8", "i16", "i32", "i64", "i128",
            "u8", "u16", "u32", "u64", "u128",
            "isize", "usize",
        ];
        INTEGERS.contains(&ident.to_string().as_str())
    }
}