mod db;
#[cfg(test)]
mod debug;
mod parse;
#[cfg(test)]
mod test;
mod tokens;
mod try_from;

pub struct DevItemFn {
    pub vis: syn::Visibility,
    pub sig: DevSignature,
    pub block: syn::Block,
}

pub struct DevSignature {
    // pub constness: Option<Const>,
    // pub asyncness: Option<Async>,
    // pub unsafety: Option<Unsafe>,
    // pub abi: Option<Abi>,
    pub fn_token: syn::Token![fn],
    pub ident: DevFnIdent,
    // pub generics: Generics,
    pub paren_token: syn::token::Paren,
    pub inputs: syn::punctuated::Punctuated<DevFnArg, syn::token::Comma>,
    // pub variadic: Option<Variadic>,
    pub output: DevReturnType,
}

pub struct DevFnIdent(pub syn::Ident);
pub struct DevFnArg {
    // pub attrs: Vec<Attribute>,
    pub pat: syn::Pat,
    pub colon_token: syn::token::Colon,
    pub ty: DevArgType,
}
pub enum DevReturnType {
    Default,
    Type(syn::token::RArrow, DevType),
    Tuple(syn::token::RArrow, DevTypeTuple),
}

pub struct DevTypeTuple {
    pub paren_token: syn::token::Paren,
    pub elems: syn::punctuated::Punctuated<DevType, syn::token::Comma>,
}

pub enum DevType {
    Float(syn::Ident), // `f{32|64}`
    Bool(syn::Ident), // `bool`
    SizedInteger(syn::Ident), // `{i|u}{8|16|32|64}`
}

pub enum DevArgType {
    DeviceType(DevType),
    SpindleAny(syn::Ident), // `Any`
    SpindleAnyFullPath(syn::punctuated::Punctuated<syn::Ident, syn::token::PathSep>) // `spindle::zst::Any`
}
