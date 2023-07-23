use proc_macro2::Ident;
use syn::{ItemFn, Signature, PatType};

use crate::camel_word;

/*  https://docs.rs/syn/latest/syn/struct.ItemFn.html
pub struct ItemFn {             A free-standing function: fn process(n: usize) -> Result<()> { ... }.
    pub attrs: Vec<Attribute>,  An attribute, like #[repr(transparent)].
        * currently we prohibit them, but they are ergonomic and useful
    pub vis: Visibility,        The visibility level of an item: inherited or pub or pub(restricted).
        * inside the ptx crate, visibility is set explicitly as needed
        * perhaps this can be used to control visibility of the trait module & contents
    pub sig: Signature,         A function signature in a trait or implementation: unsafe fn initialize(&self).
        * see below
    pub block: Box<Block>,      A braced block containing Rust statements.
        * open question: what to restrict? add compiler warnings?
}
*/

pub(crate) trait RegulateItemFn: Sized {
    fn no_attributes(self) -> Result<Self, &'static str>;
    fn no_generics(self) -> Result<Self, &'static str>;
    fn no_where_clause(self) -> Result<Self, &'static str>;
}

static NO_ATTRIBUTES: &str = "no attributes";
static NO_GENERICS: &str = "no generics";
static NO_WHERE_CLAUSE: &str = "no where clauses";

impl RegulateItemFn for ItemFn {
    fn no_attributes(self) -> Result<Self, &'static str> {
        if !self.attrs.is_empty() {
            return Err(NO_ATTRIBUTES);
        } else {
            Ok(self)
        }
    }

    fn no_generics(self) -> Result<Self, &'static str> {
        if !self.sig.generics.params.is_empty() {
            return Err(NO_GENERICS);
        } else {
            Ok(self)
        }
    }

    fn no_where_clause(self) -> Result<Self, &'static str> {
        if self.sig.generics.where_clause.is_some() {
            return Err(NO_WHERE_CLAUSE);
        } else {
            Ok(self)
        }
    }
}

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
*/

static NAME_HEAD: &str = "lower snake case w/ at most one leading _ (must be followed by a letter)";
static NAME_TAIL: &str = "lower snake case w/ no trailing _";

pub(crate) trait RegulateIdent: Sized {
    fn at_most_one_leading_underscore(self) -> Result<Self, &'static str>;
    fn no_trailing_underscores(self) -> Result<Self, &'static str>;
    fn trimmed_lower_snake_to_trimmed_upper_camel(self) -> Result<(Self, Self), &'static str>;
    // fn trimmed_upper_camel_to_trimmed_lower_snake(self) -> Result<(Self, Self), &'static str>;
        // eventually for structs, enums, etc.
}

impl RegulateIdent for Ident {
    fn at_most_one_leading_underscore(self) -> Result<Self, &'static str> {
        let name = self.to_string();
        if name.starts_with("__") {
            return Err(NAME_HEAD);
        }
        Ok(self)
    }

    fn no_trailing_underscores(self) -> Result<Self, &'static str> {
        let name = self.to_string();
        if name.ends_with('_') {
            return Err(NAME_TAIL);
        }
        Ok(self)
    }

    fn trimmed_lower_snake_to_trimmed_upper_camel(self) -> Result<(Self, Self), &'static str> {
        let name = self.to_string();
        let words = name.split('_');
        let camel_words = words.map(camel_word).collect::<Vec<_>>();
        let camel = camel_words.join("");
        let camel = Ident::new(&camel, self.span());
        Ok((self, camel))
    }
}


/* (Signature, continued)
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
    fn no_variadic(self) -> Result<Self, &'static str>;
    fn typed_args(&self) -> Result<Vec<&PatType>, &'static str>;
}

static NO_CONST: &str = "no const fns";
static NO_ASYNC: &str = "no async fns";
static NO_ABI: &str = "no abi on fn";
static NO_VARIADICS: &str = "no variadics";
static NOT_A_METHOD: &str = "may not be a method";

impl RegulateSignature for Signature {
    fn no_const(self) -> Result<Self, &'static str> {
        if self.constness.is_some() {
            return Err(NO_CONST);
        } else {
            Ok(self)
        }
    }

    fn no_async(self) -> Result<Self, &'static str> {
        if self.asyncness.is_some() {
            return Err(NO_ASYNC);
        } else {
            Ok(self)
        }
    }

    fn no_abi(self) -> Result<Self, &'static str> {
        if self.abi.is_some() {
            return Err(NO_ABI);
        } else {
            Ok(self)
        }
    }

    fn no_generics(self) -> Result<Self, &'static str> {
        if !self.generics.params.is_empty() {
            return Err(NO_GENERICS);
        } else {
            Ok(self)
        }
    }

    fn no_variadic(self) -> Result<Self, &'static str> {
        if self.variadic.is_some() {
            return Err(NO_VARIADICS);
        } else {
            Ok(self)
        }
    }
    
    fn typed_args(&self) -> Result<Vec<&PatType>, &'static str> {
        self.inputs.iter().map(|arg| match arg {
            syn::FnArg::Receiver(_) => Err(NOT_A_METHOD),
            syn::FnArg::Typed(arg) => Ok(arg),
        }).collect()
    }
}

/* pub struct PatType { https://docs.rs/syn/latest/syn/struct.PatType.html
    pub attrs: Vec<Attribute>,  An attribute, like #[repr(transparent)].
        * prohibit
    
    pub pat: Box<Pat>,  https://docs.rs/syn/latest/syn/enum.Pat.html
        A pattern in a local binding, function signature, match expression, or various other places.
        ❌ Const(PatConst)      A const block: const { ... }.
        ❌ Ident(PatIdent)      A pattern that binds a new variable: ref mut binding @ SUBPATTERN.
        ❌ Lit(PatLit)          A literal in place of an expression: 1, "foo".
        ❌ Macro(PatMacro)      A macro invocation expression: format!("{}", q).
        ❌ Or(PatOr)            A pattern that matches any one of a set of cases.
        ❌ Paren(PatParen)      A parenthesized pattern: (A | B).
        🤔 Path(PatPath)        A path like std::mem::replace possibly containing generic parameters and a qualified self-type.
                                A plain identifier like x is a path of length 1.
        ❌ Range(PatRange)      A range expression: 1..2, 1.., ..2, 1..=2, ..=2.
        🤔 Reference(PatReference)
                                A reference pattern: &mut var.
        ❌ Rest(PatRest)        The dots in a tuple or slice pattern: [0, 1, ..].
        ❌ Slice(PatSlice)      A dynamically sized slice pattern: [a, b, ref i @ .., y, z].
        ❌ Struct(PatStruct)    A struct or struct variant pattern: Variant { x, y, .. }.
        🤔 Tuple(PatTuple)      A tuple pattern: (a, b).
        🤔 TupleStruct(PatTupleStruct)
                                A tuple struct or tuple variant pattern: Variant(x, y, .., z).
        🚧 Type(PatType)        A type ascription pattern: foo: f64.
        ❌ Verbatim(TokenStream)
                                An abstract stream of tokens, or more concretely a sequence of token trees.
        ❌ Wild(PatWild)        A pattern that matches any value: _.
    pub colon_token: Colon,     Don’t try to remember the name of this type — use the Token! macro instead.
    
    pub ty: Box<Type>,  https://docs.rs/syn/latest/syn/enum.Type.html
        The possible types that a Rust value could have.
        🚧 Array(TypeArray)     A fixed size array type: [T; n].
        ❌ BareFn(TypeBareFn)   A bare function type: fn(usize) -> bool.
        ❌ Group(TypeGroup)     A type contained within invisible delimiters.
        ❌ ImplTrait(TypeImplTrait)
                                An impl Bound1 + Bound2 + Bound3 type where Bound is a trait or a lifetime.
        ❌ Infer(TypeInfer)     Indication that a type should be inferred by the compiler: _.
        ❌ Macro(TypeMacro)     A macro in the type position.
        ❌ Never(TypeNever)     The never type: !.
        ❌ Paren(TypeParen)     A parenthesized type equivalent to the inner type.
        🚧 Path(TypePath)       A path like std::slice::Iter, optionally qualified with a self-type as in <Vec<T> as SomeTrait>::Associated.
        ❌ Ptr(TypePtr)         A raw pointer type: *const T or *mut T.
        🚧 Reference(TypeReference)
                                A reference type: &'a T or &'a mut T.
        ❌ Slice(TypeSlice)     A dynamically sized slice type: [T].
        ❌ TraitObject(TypeTraitObject)
                                A trait object type dyn Bound1 + Bound2 + Bound3 where Bound is a trait or a lifetime.
        🤔 Tuple(TypeTuple)     A tuple type: (A, B, C, String).
        ❌ Verbatim(TokenStream)
                                An abstract stream of tokens, or more concretely a sequence of token trees.
} */

/* regulations for types:
    * start with primitive types (i32, usize, f32, etc.)
    * 🚧 work in [T; n] and then [T; N]
    * 🤔 think about what we want from tuples, structs, enums, etc.
    * ❌ explicitly forbid () types
*/

/* plan for lifts
    [range function initializers]
        fn foo(n: i32) ➡️ X constructs
        DevSpindle<U, X>
            U: RawConvert<X>
        from a range, e.g., 30..100_030
    [univariate function]
        fn foo(x: X) ➡️ Y lifts to
        DevSpindle<U, X> ➡️
        DevSpindle<U, Y>
            U: RawConvert<X> + RawConvert<Y>
    [univariate function with an ⚠️ immutible reference]
        fn foo(x: X, a: &A) ➡️ Y lifts to
        DevSpindle<U, X> ➕ &DevState<S> ➡️
        DevSpindle<U, Y>
            U: RawConvert<X> ➕ RawConvert<Y>,
            S: RawConvert<A>
        ⚠️ immutability is required since the "state" S is shared by threads
        ⚠️ note that pure functions have arguments mapped in a "mutable" way already
    [multivariate function]
        fn foo(w: W, x: X) ➡️ (Y, Z) lifts to
        DevSpindle<U, W> ➕ DevSpindle<V, X> ➡️
        DevSpindle<U, Y> ➕ DevSpindle<V, Z>
            U: RawConvert<W> + RawConvert<Y>,
            V: RawConvert<X> + RawConvert<Z>,
    [multivariate function w/ optional inplace mutability]
        fn foo(w: W, x: X) ➡️ (Y, ()) lifts to
        DevSpindle<U, W> ➕ DevSpindle<V, X> ➡️
        DevSpindle<U, Y>
            U: RawConvert<W> + RawConvert<Y>,
            V: RawConvert<X>,
*/

static EXACTLY_ONE_INPUT: &str = "exactly one (integer) input";
static ONLY_INTEGERS: &str = "only integer inputs (isize, usize, i32, u32, etc.)";
static NO_RETURN: &str = "missing return type";
static ONLY_PRIMITIVE_RETURNS: &str = "only returns primitive numbers (i32, usize, f32, etc.)";
static ONLY_I32: &str = "range functions currently only admit i32";

pub(crate) trait RegulatePatTypes: Sized {

}
