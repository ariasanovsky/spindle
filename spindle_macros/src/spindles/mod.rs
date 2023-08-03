use uuid::Uuid;

use crate::{maps::Map, unions::_Union};

struct Spindle {
    id: Uuid,
    unions: Vec<_Union>,
    maps: Vec<Map>,
}
