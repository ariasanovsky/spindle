use std::fmt::Display;

use spindle_db::tag::AsDbTag;

use crate::case::LowerSnakeIdent;

#[derive(Clone, Debug)]
pub struct CrateTag(pub LowerSnakeIdent);

impl PartialEq for CrateTag {
    fn eq(&self, other: &Self) -> bool {
        self.0.0.to_string() == other.0.0.to_string()
    }
}

impl AsDbTag for CrateTag {
    fn db_tag(&self) -> String {
        self.0 .0.to_string()
    }
}

impl Display for CrateTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 .0)
    }
}
