use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{maps::Map, unions::_Union};

#[derive(Clone, Serialize, Deserialize)]
struct Spindle {
    id: Uuid,
    unions: Vec<_Union>,
    maps: Vec<Map>,
}
