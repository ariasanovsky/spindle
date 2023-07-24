pub(crate) mod ident;
pub(crate) mod signature;
pub(crate) mod pat_type;
pub(crate) mod return_type;
pub(crate) mod item_fn;

pub(crate) static UNEXPECTED_ATTRIBUTES: &str = "no attributes";
pub(crate) static UNEXPECTED_GENERICS: &str = "no generics";
pub(crate) static UNEXPECTED_WHERE_CLAUSE: &str = "no where clauses";

pub(crate) static SNAKE_NAME_HEAD: &str = "lower snake case w/ at most one leading _ (must be followed by a letter)";
pub(crate) static SNAKE_NAME_TAIL: &str = "lower snake case w/ no trailing _";

pub(crate) static UNEXPECTED_CONST: &str = "unexpected const token";
pub(crate) static UNEXPECTED_ASYNC: &str = "unexpected async token";
pub(crate) static UNEXPECTED_ABI: &str = "unexpected abi tokens";
pub(crate) static UNEXPECTED_VARIADICS: &str = "unexpected variadics";
pub(crate) static UNEXPECTED_SELF: &str = "unexpected self -- methods not supported";

pub(crate) static EXPECTED_INPUT_ONE: &str = "expected exactly one input";
pub(crate) static EXPECTED_INPUTS_INDENT: &str = "expected only explicit named types as inputs";
pub(crate) static EXPECTED_ONE_INPUT_PRIMITIVE: &str = "expected exactly one input";
pub(crate) static EXPECTED_ONE_INPUT_INTEGER: &str = "expected exactly one integer input";
pub(crate) static _EXPECTED_ONE_INPUT_INTEGER_I32: &str = "expected exactly one i32 input";
// pub(crate) static _ONLY_INTEGERS: &str = "only integer inputs (isize, usize, i32, u32, etc.)";

// pub(crate) static NO_RETURN: &str = "missing return type";
pub(crate) static EXPECTED_RETURN_IDENT: &str = "expected an ident return type";
pub(crate) static UNEXPECTED_RETURN: &str = "expected no return type";
pub(crate) static EXPECTED_RETURN_PRIMITIVE: &str = "expeected a primitive return (i32, usize, f64, etc.)";

pub(crate) static ARRAYS_SOON: &str = "arrays will be soon!";
pub(crate) static REFERENCES_SOON: &str = "references will be soon!";
pub(crate) static TUPLES_SOON: &str = "tuples will be soon!";

pub(crate) static UNSUPPORTED_RETURN: &str = "unsupported return type -- suggestions welcome at ";

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

