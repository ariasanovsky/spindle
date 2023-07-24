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

use proc_macro2::Ident;
use syn::PatType;

use super::{EXPECTED_ONE_INPUT_INTEGER, UNEXPECTED_ATTRIBUTES, EXPECTED_INPUTS_INDENT};

pub(crate) trait RegulatePatTypes: Sized {
    fn exactly_one_input(self) -> Result<Self, &'static str>;
    fn only_ident_inputs(&self) -> Result<Vec<Ident>, &'static str>;
}

impl RegulatePatTypes for Vec<PatType> {
    fn exactly_one_input(self) -> Result<Self, &'static str> {
        if self.len() == 1 {
            Ok(self)
        } else {
            Err(EXPECTED_ONE_INPUT_INTEGER)
        }
    }

    fn only_ident_inputs(&self) -> Result<Vec<Ident>, &'static str> {
        self.iter().map(|arg| {
            if !arg.attrs.is_empty() {
                return Err(UNEXPECTED_ATTRIBUTES);
            }
            // todo! what to do with `pat: Pat`?
            let type_path = match arg.ty.as_ref() {
                syn::Type::Path(type_path) => type_path.clone(),
                _ => return Err(EXPECTED_INPUTS_INDENT),
            };
            // The explicit Self type in a qualified path: the T in <T as Display>::fmt.
            if type_path.qself.is_some() {
                return Err(EXPECTED_INPUTS_INDENT);
            }
            // A path like std::slice::Iter, optionally qualified with a self-type as in <Vec<T> as SomeTrait>::Associated.
            type_path.path.get_ident().cloned().ok_or(EXPECTED_INPUTS_INDENT)
        }).collect()
    }
}
