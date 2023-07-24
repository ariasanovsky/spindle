use syn::ItemFn;

use super::{UNEXPECTED_ATTRIBUTES, UNEXPECTED_GENERICS, UNEXPECTED_WHERE_CLAUSE};

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

impl RegulateItemFn for ItemFn {
    fn no_attributes(self) -> Result<Self, &'static str> {
        if !self.attrs.is_empty() {
            Err(UNEXPECTED_ATTRIBUTES)
        } else {
            Ok(self)
        }
    }

    fn no_generics(self) -> Result<Self, &'static str> {
        if !self.sig.generics.params.is_empty() {
            Err(UNEXPECTED_GENERICS)
        } else {
            Ok(self)
        }
    }

    fn no_where_clause(self) -> Result<Self, &'static str> {
        if self.sig.generics.where_clause.is_some() {
            Err(UNEXPECTED_WHERE_CLAUSE)
        } else {
            Ok(self)
        }
    }
}
