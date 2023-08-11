use proc_macro2::Ident;

mod parse;
#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct LowerSnakeIdent(pub Ident);

#[derive(Debug, Clone)]
pub struct UpperCamelIdent(pub Ident);

#[derive(Debug, Clone)]
pub struct PrimitiveIdent(pub Ident);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Case {
    LowerSnake,
    UpperCamel,
    SupportedPrimitive,
    UnsupportedPrimitive,
    Unknown,
}

pub trait Cased {
    fn case(&self) -> Case;
    fn split_underscores(&self) -> (usize, Option<&str>);
    fn rsplit_underscores(&self) -> (Option<&str>, usize);
}

impl Cased for &str {
    fn case(&self) -> Case {
        use heck::{ToUpperCamelCase, ToSnekCase};
        let camel = self.to_upper_camel_case();
        let is_camel = &camel == self;
        let snake = self.to_snek_case();
        let is_snake = &snake == self;
        match (is_camel, is_snake) {
            (true, false) => Case::UpperCamel,
            (false, true) => {
                const SUPPORTED_PRIMITIVES: &[&str] = &[
                    "bool", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "str", "u8",
                    "u16", "u32", "u64", "u128",
                ];
                if SUPPORTED_PRIMITIVES.contains(&self) {
                    Case::SupportedPrimitive
                } else {
                    const UNSUPPORTED_PRIMITIVES: &[&str] = &["usize", "isize", "char"];
                    if UNSUPPORTED_PRIMITIVES.contains(&self) {
                        Case::UnsupportedPrimitive
                    } else {
                        Case::LowerSnake
                    }
                }
            },
            _ => Case::Unknown,
        }
    }

    fn split_underscores(&self) -> (usize, Option<&str>) {
        self
            .find(|c| c != '_')
            .map_or((self.len(), None), |i| (i, Some(&self[i..])))
    }

    fn rsplit_underscores(&self) -> (Option<&str>, usize) {
        self
            .rfind(|c| c != '_')
            .map_or((None, self.len()), |i| (Some(&self[..=i]), self.len() - i - 1))
    }
}
