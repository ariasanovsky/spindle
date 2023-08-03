mod test;

/*
* associate f: X -> Y w/ all U s.t.
    * U is a union
    * X, Y are fields of U
* associate f: (X, A) -> (Y, B) w/ (U, V) s.t.
    * U, V are unions
    * X, Y are fields of U
    * A, B are fields of V
* we associate f: X -> Y w/
    * the in_out pair (X, Y)
    * via a junction table
* we associate U w/
    * each field of U
    * via a junction table
*/

use crate::{union::DbUnion, map::DbMap, TypeDb, DbResult};

/* created by spin!(U = f32 | u64, V)
    * extracts [
        U = f32 | u64, 
        V = i32 | bool | etc    [db lookup]
    ] // the db lookup happens before constructing the crate
    * finds all f: (X, A) -> (Y, B) s.t.
        * X, Y in {f32, u64}        [U]
        * A, B in {f64, i32, etc}   [V]
    * for each f, writes
        * a kernel          `f_kernel   *mut U, *mut V, i32`
        * a (U, V) method   `(U, V)::f` &mut self
        * a device function `f`         (X, A) -> (Y, B)
*/
pub struct SpindleCrate {
    pub(crate) uuid: String,
    pub(crate) unions: Vec<DbUnion>,
    pub(crate) maps: Vec<DbMap>,
}

impl TypeDb {
    pub fn get_or_insert_crate_from_unions(&self, unions: &[DbUnion]) -> DbResult<SpindleCrate> {
        todo!()
    }
}