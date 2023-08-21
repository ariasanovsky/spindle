#[cfg(test)]
mod test;
mod try_from;

pub struct DeviceItemFn {
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

pub struct DevFnIdent;
pub struct DevFnArg;
pub struct DevReturnType;
