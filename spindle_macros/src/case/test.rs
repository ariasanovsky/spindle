use crate::case::{Case, Cased};

#[test]
fn underscore_splits_correct_with_no_underscores() {
    let s = "foo_bar";
    assert_eq!(s.split_underscores(), (0, Some(s)));
    assert_eq!(s.rsplit_underscores(), (Some(s), 0));
}

#[test]
fn underscore_splits_correct_with_one_underscore() {
    let s = "_foo_bar_";
    assert_eq!(s.split_underscores(), (1, Some("foo_bar_")));
    assert_eq!(s.rsplit_underscores(), (Some("_foo_bar"), 1));
}

#[test]
fn cases_identified_correctly() {
    let s = "foo_bar";
    assert_eq!(s.case(), Case::LowerSnake);
    let s = "FooBar";
    assert_eq!(s.case(), Case::UpperCamel);
    let s = "U";
    assert_eq!(s.case(), Case::UpperCamel);
    let s = "i32";
    assert_eq!(s.case(), Case::SupportedPrimitive);
    let s = "char";
    assert_eq!(s.case(), Case::UnsupportedPrimitive);
}
