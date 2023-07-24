use syn::{PatType, Signature};

use super::{
    UNEXPECTED_ABI, UNEXPECTED_ASYNC, UNEXPECTED_CONST, UNEXPECTED_GENERICS, UNEXPECTED_SELF,
    UNEXPECTED_VARIADICS,
};

/*  https://docs.rs/syn/latest/syn/struct.Signature.html
pub struct Signature {
    pub constness: Option<Const>,   Don’t try to remember the name of this type — use the Token! macro instead.
        * forbid for now?
    pub asyncness: Option<Async>,   Don’t try to remember the name of this type — use the Token! macro instead.
        * forbid for now
    pub unsafety: Option<Unsafe>,   Don’t try to remember the name of this type — use the Token! macro instead.
        * allowed, transfer fine to the ptx crate
    pub abi: Option<Abi>,           The binary interface of a function: extern "C".
        * forbid for now
    pub fn_token: Fn,               Don’t try to remember the name of this type — use the Token! macro instead.
    pub ident: Ident,               A word of Rust code, which may be a keyword or legal variable name.
        * in the host crate, we do not alter the name, the ergonomics of that sound dubious
        * we require lower snake case (name collisions, warnings)
        * we prohibit __{.*} and {.*}_ names (name collisions)
        * in the ptx crate, the optional leading _ is trimmed
        * in the host crate, the sanitary module is named __{trimmed_name}
        * in the sanitary module, the spindle trait is named upper_camel(trimmed_name)
    pub generics: Generics,         Lifetimes and type parameters attached to a declaration of a function, enum, trait, etc.
        * forbid for now -- perhaps array length parameters could be useful?
    pub paren_token: Paren,         (…)
    pub inputs: Punctuated<         A punctuated sequence of syntax tree nodes of type T separated by punctuation of type P.
        * useful to regulate the number of inputs
        FnArg,                      An argument in a function signature: the n: usize in fn f(n: usize).
            * see below, FnArg is a very sensitive topic and vital we get it right
        Comma                       Don’t try to remember the name of this type — use the Token! macro instead.
    >,
    pub variadic: Option<Variadic>, The variadic argument of a foreign function. `fn printf(format: *const c_char, ...) -> c_int;`
        * forbid for now, what's the use case?
    pub output: ReturnType,         Return type of a function signature.
        * see below, ReturnType is a very sensitive topic and vital we get it right

    pub fn receiver(&self) -> Option<&Receiver>
        ... A method’s self receiver, such as &self or self: Box<Self>.
        * forbid for now
} */

pub(crate) trait RegulateSignature: Sized {
    fn no_const(self) -> Result<Self, &'static str>;
    fn no_async(self) -> Result<Self, &'static str>;
    fn no_abi(self) -> Result<Self, &'static str>;
    fn no_generics(self) -> Result<Self, &'static str>;
    fn only_typed_inputs(&self) -> Result<Vec<&PatType>, &'static str>;
    fn no_variadic(self) -> Result<Self, &'static str>;
}

impl RegulateSignature for Signature {
    fn no_const(self) -> Result<Self, &'static str> {
        if self.constness.is_some() {
            Err(UNEXPECTED_CONST)
        } else {
            Ok(self)
        }
    }

    fn no_async(self) -> Result<Self, &'static str> {
        if self.asyncness.is_some() {
            Err(UNEXPECTED_ASYNC)
        } else {
            Ok(self)
        }
    }

    fn no_abi(self) -> Result<Self, &'static str> {
        if self.abi.is_some() {
            Err(UNEXPECTED_ABI)
        } else {
            Ok(self)
        }
    }

    fn no_generics(self) -> Result<Self, &'static str> {
        if !self.generics.params.is_empty() {
            Err(UNEXPECTED_GENERICS)
        } else {
            Ok(self)
        }
    }

    fn only_typed_inputs(&self) -> Result<Vec<&PatType>, &'static str> {
        self.inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Receiver(_) => Err(UNEXPECTED_SELF),
                syn::FnArg::Typed(arg) => Ok(arg),
            })
            .collect()
    }

    fn no_variadic(self) -> Result<Self, &'static str> {
        if self.variadic.is_some() {
            Err(UNEXPECTED_VARIADICS)
        } else {
            Ok(self)
        }
    }
}
