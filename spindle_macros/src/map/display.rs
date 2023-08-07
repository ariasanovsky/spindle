use std::fmt::Display;

use super::MapFn;

impl Display for MapFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!();
        write!(f, "{}", self.item_fn.sig.ident);
        todo!()
    }
}