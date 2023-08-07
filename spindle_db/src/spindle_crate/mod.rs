mod display;
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

use rusqlite::OptionalExtension;

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
#[derive(Clone, Debug)]
pub struct DbCrate {
    pub(crate) uuid: String,
    pub(crate) unions: Vec<DbUnion>,            // [U_1, ..., U_m]
    pub(crate) lifts: Vec<DbLiftGivenUnions>,   // vec of [f: (X_1, ..., X_m) -> (Y_1, ..., Y_m)]
}

impl DbCrate {
    pub(crate) fn new(unions: Vec<DbUnion>, lifts: Vec<DbLiftGivenUnions>) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            unions,
            lifts,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbLiftGivenUnions {
    pub(crate) uuid: String,
    pub(crate) map: DbMap,
    pub(crate) positions: Vec<(Option<usize>, Option<usize>)>,
}

impl DbLiftGivenUnions {
    pub(crate) fn new(map: DbMap, positions: Vec<(Option<usize>, Option<usize>)>) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            map,
            positions,
        }
    }
}

impl TypeDb {
    pub fn get_or_insert_crate_from_unions(&self, unions: Vec<DbUnion>) -> DbResult<DbCrate> {
        let lifts = self.get_maps_from_unions(&unions)?;
        let db_crate: Option<DbCrate> = self.get_crate_from_unions_and_lifts(&unions, &lifts)?;
        match db_crate {
            Some(db_crate) => Ok(db_crate),
            None => {
                let db_crate: DbResult<DbCrate> = self.insert_crate_from_unions_and_lifts(unions, lifts);
                db_crate
            }
        }
    }

    fn get_crate_from_unions_and_lifts(
        &self,
        unions: &[DbUnion],
        lifts: &[DbLiftGivenUnions]
    ) -> DbResult<Option<DbCrate>> {
        // todo! overkill
        // first get all crates
        let mut crates: Vec<DbCrate> = self.get_all_crates()?;
        // then filter out crates by unions and lifts
        crates.retain(|db_crate| {
            db_crate.unions == unions && db_crate.lifts == lifts
        });
        // panic if there is more than one crate
        assert!(crates.len() <= 1, "critical error: more than one crate found: {:?}", crates);
        Ok(crates.into_iter().next())
    }

    fn get_all_crates(&self) -> DbResult<Vec<DbCrate>> {
        let mut statement = self.conn.prepare("SELECT uuid FROM crates")?;
        let crates: DbResult<Vec<_>> = statement.query_map([], |row| {
            let uuid: String = row.get(0)?;
            let db_crate: DbResult<DbCrate> = self.get_crate_from_uuid(uuid);
            db_crate
        })?.collect();
        crates
    }

    fn get_crate_from_uuid(&self, uuid: String) -> DbResult<DbCrate> {
        // first we collect the Vec<(usize, DbUnion)> associated with the crate, sorted by position
        let mut statement = self.conn.prepare("
            SELECT pos, union_uuid FROM crate_unions WHERE crate_uuid = ? ORDER BY pos
        ")?;
        let unions = statement.query_map([&uuid], |row| {
            let pos: usize = row.get(0)?;
            let union_uuid: String = row.get(1)?;
            let db_union: DbResult<DbUnion> = self.get_union_from_uuid(union_uuid).transpose().expect("union not found");
            db_union.map(|db_union| (pos, db_union))
        })?;
        let unions: DbResult<Vec<DbUnion>> = unions.enumerate().map(|(i, row)| {
            let (pos, db_union) = row?;
            assert_eq!(pos, i, "critical error: union positions are not consecutive");
            Ok(db_union)
        }).collect();
        // next we collect the Vec<(usize, DbLiftGivenUnions)> associated with the crate, sorted by position
        let mut statement = self.conn.prepare("
            SELECT pos, lift_uuid FROM lift_crates WHERE crate_uuid = ? ORDER BY pos
        ")?;
        let lifts = statement.query_map([&uuid], |row| {
            let pos: usize = row.get(0)?;
            let lift_uuid: String = row.get(1)?;
            let lift: DbResult<DbLiftGivenUnions> = self.get_lift_from_uuid(lift_uuid).transpose().expect("lift not found");
            lift.map(|lift| (pos, lift))
        })?;
        let lifts: DbResult<Vec<DbLiftGivenUnions>> = lifts.enumerate().map(|(i, row)| {
            let (pos, lift) = row?;
            assert_eq!(pos, i, "critical error: lift positions are not consecutive");
            Ok(lift)
        }).collect();
        // finally we return the crate
        let db_crate = DbCrate {
            uuid,
            unions: unions?,
            lifts: lifts?,
        };
        Ok(db_crate)
    }

    fn get_lift_from_uuid(&self, uuid: String) -> DbResult<Option<DbLiftGivenUnions>> {
        dbg!();
        // first we get the associated map
        let mut statement = self.conn.prepare("
            SELECT map_uuid FROM lifts WHERE uuid = ?
        ")?;
        // funny monad business
        let map: DbResult<Option<DbMap>> = statement.query_row([&uuid], |row| {
            let map_uuid: String = row.get(0)?;
            self.get_map_from_uuid(map_uuid)
        }).optional();
        let map = if let Some(map) = map? {
            map
        } else {
            return Ok(None);
        };
        // since we got a map, we better have a nonempty vec of positions (pos, in_pos, out_pos)
        let mut statement = self.conn.prepare("
            SELECT pos, in_pos, out_pos FROM lift_maps WHERE lift_uuid = ? ORDER BY pos
        ")?;
        let positions = statement.query_map([&uuid], |row| {
            let pos: usize = row.get(0)?;
            let in_pos: Option<usize> = row.get(1)?;
            let out_pos: Option<usize> = row.get(2)?;
            Ok((pos, in_pos, out_pos))
        })?;
        let positions: Vec<(Option<usize>, Option<usize>)> = positions.into_iter().enumerate().map(|(i, row)| {
            let (pos, in_pos, out_pos) = row?;
            assert_eq!(pos, i, "critical error: lift map positions are not consecutive");
            Ok((in_pos, out_pos))
        }).collect::<DbResult<_>>()?;
        // finally we return the lift
        let db_lift = DbLiftGivenUnions {
            uuid,
            map,
            positions,
        };
        Ok(Some(db_lift))
    }

    fn insert_crate_from_unions_and_lifts(
        &self,
        unions: Vec<DbUnion>,
        lifts: Vec<DbLiftGivenUnions>
    ) -> DbResult<DbCrate> {
        // create a new crate
        let db_crate = DbCrate::new(unions, lifts);
        dbg!();
        // insert crate, then join with unions and lifts
        let mut statement = self.conn.prepare("
            INSERT INTO crates (uuid) VALUES (?)
        ")?;
        let _: usize = statement.execute([&db_crate.uuid])?;
        dbg!();
        // insert unions
        let mut statement = self.conn.prepare("
            INSERT INTO crate_unions (crate_uuid, pos, union_uuid) VALUES (?, ?, ?)
        ")?;
        let _: () = db_crate.unions.iter().enumerate().map(|(i, db_union)| {
            let _: usize = statement.execute(rusqlite::params![&db_crate.uuid, i, &db_union.uuid])?;
            Ok(())
        }).collect::<DbResult<()>>()?;
        dbg!();
        // insert lifts
        let mut statement = self.conn.prepare("
            INSERT INTO lift_crates (crate_uuid, pos, lift_uuid) VALUES (?, ?, ?)
        ")?;
        let _: () = db_crate.lifts.iter().enumerate().map(|(i, db_lift)| {
            let _: usize = statement.execute(rusqlite::params![&db_crate.uuid, i, &db_lift.uuid])?;
            Ok(())
        }).collect::<DbResult<()>>()?;
        dbg!();
        // return the crate
        Ok(db_crate)
    }

    fn get_maps_from_unions(&self, unions: &[DbUnion]) -> DbResult<Vec<DbLiftGivenUnions>> {
        // collect all maps `f: (X_1, ..., X_m) -> (Y_1, ..., Y_m)` s.t.
        // for all 1 <= i <= m, X_i, Y_i are fields of U_i
        // here U_i is a the i-th union in `unions`
        // we also collect the positions of the X_i and Y_i w.r.t. U_i
        // first, we collect all maps
        let mut maps = self.get_maps()?;
        // next we retain only the maps with the right number of in_outs
        maps.retain(|map| {
            map.in_outs.len() == unions.len()
        });
        // finally, we filter maps by the in_outs
        let maps: DbResult<Vec<DbLiftGivenUnions>> = maps.into_iter().filter_map(|map| {
            // for each in_out, we check if the in and the out are fields of the corresponding union
            // we log their positions as Option<usize>, allowing the None case iff the in (out) is a None
            let foo = map.in_outs.iter().zip(unions.iter());
            let positions: Option<Vec<(Option<usize>, Option<usize>)>> = foo.map(|(in_out, u)| {
                // &Option<_> is useless filth
                // Option<&_> is your best friend
                // todo! ?style points to transpose and ? sugar
                let in_field = in_out.0.as_ref();
                let out_field = in_out.1.as_ref();
                let in_position: Option<usize> = match in_field {
                    Some(in_field) => Some(u.fields.iter().position(|field| field == in_field)?),
                    None => None,
                };
                let out_position: Option<usize> = match out_field {
                    Some(out_field) => Some(u.fields.iter().position(|field| field == out_field)?),
                    None => None,
                };
                Some((in_position, out_position))
            }).collect();
            let lift: Option<DbResult<DbLiftGivenUnions>> = positions.map(|positions| {
                let lift: DbResult<DbLiftGivenUnions> = self.get_or_insert_lift_given_unions(map, positions, &unions);
                return lift;
            });
            lift
        }).collect::<DbResult<Vec<_>>>();
        maps
    }

    fn get_or_insert_lift_given_unions(
        &self,
        map: DbMap,
        positions: Vec<(Option<usize>, Option<usize>)>,
        unions: &[DbUnion]
    ) -> DbResult<DbLiftGivenUnions> {
        let lift: Option<DbLiftGivenUnions> = self.get_lift_given_unions(&map, &positions, &unions)?;
        match lift {
            Some(lift) => Ok(lift),
            None => {
                // call the insert fn
                let lift: DbResult<DbLiftGivenUnions> = self.insert_lift_given_unions(map, positions, unions);
                lift
            }
        }
    }

    fn get_lift_given_unions(
        &self,
        map: &DbMap,
        positions: &[(Option<usize>, Option<usize>)],
        unions: &[DbUnion]
    ) -> DbResult<Option<DbLiftGivenUnions>> {
        // check lifts table by map uuid
        let mut lifts: Vec<DbLiftGivenUnions> = self.get_lifts_given_map(&map.uuid)?;
        // filter lifts by number of positions
        lifts.retain(|lift| {
            lift.positions.len() == unions.len()
        });
        // filter lifts by positions
        lifts.retain(|lift| {
            lift.positions.iter().copied().zip(positions.iter().copied()).all(|(lift_pos, pos)| {
                lift_pos == pos
            })
        });
        // `lifts` either has 0 or 1 elements (else, critical error, we'll panic for now)
        assert!(lifts.len() <= 1, "Critical error: more than one lift found for map {:?}", &map);
        let lift: Option<DbLiftGivenUnions> = lifts.into_iter().next();
        Ok(lift)
    }

    fn get_lifts_given_map(&self, map_uuid: &str) -> DbResult<Vec<DbLiftGivenUnions>> {
        let mut statement = self.conn.prepare("SELECT uuid from lifts WHERE map_uuid = ?")?;
        let rows: DbResult<Vec<_>> = statement.query_map([map_uuid], |row| {
            let uuid: String = row.get(0)?;
            let lift: DbResult<DbLiftGivenUnions> = self.get_lift_from_uuid(uuid).transpose().expect("Critical error: lift uuid not found");
            lift
        })?.collect::<DbResult<Vec<_>>>();
        rows
    }

    fn insert_lift_given_unions(
        &self,
        map: DbMap,
        positions: Vec<(Option<usize>, Option<usize>)>,
        unions: &[DbUnion]
    ) -> DbResult<DbLiftGivenUnions> {
        let lift = DbLiftGivenUnions::new(map, positions);
        // ensure that the vec & slice have the same length
        assert_eq!(lift.positions.len(), unions.len(), "Critical error: lift positions and unions have different lengths");
        // add lift to the table
        let mut statement = self.conn.prepare("
            INSERT INTO lifts (uuid, map_uuid) VALUES (?, ?)
        ")?;
        let _: usize = statement.execute([&lift.uuid, &lift.map.uuid])?;
        // gather the table entries
        lift.positions
            .iter()
            .zip(unions.iter())
            .enumerate()
            .try_for_each::<_, DbResult<()>>(|(pos, ((in_pos, out_pos), union))| {
                let union_uuid = &union.uuid;
                // dbg!(&pos, &in_pos, &out_pos, &union_uuid);
                // add (lift_uuid, pos, union_uuid, in_pos, out_pos) to the table
                let mut statement = self.conn.prepare("
                    INSERT INTO lift_entries (lift_uuid, pos, union_uuid, in_pos, out_pos) VALUES (?, ?, ?, ?, ?)
                ")?;
                let _: usize = statement.execute(rusqlite::params![lift.uuid, pos, union_uuid, in_pos, out_pos])?;
                Ok(())
            })?;
        Ok(lift)
    }
}
