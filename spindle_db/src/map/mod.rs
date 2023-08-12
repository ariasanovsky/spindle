use crate::{
    primitive::{AsDbPrimitive, DbPrimitive},
    DbResult, TypeDb, tag::AsDbTag,
};

pub mod iter;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Eq)]
pub struct DbMap {
    pub uuid: String,
    pub ident: String,
    pub content: String,
    pub in_outs: Vec<DbInOut>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DbInOut {
    pub input: Option<DbPrimitive>,
    pub output: Option<DbPrimitive>,
}

impl PartialEq for DbMap {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.in_outs == other.in_outs
    }
}

impl DbMap {
    pub(crate) fn new(ident: String, content: String, in_outs: Vec<DbInOut>) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            ident,
            content,
            in_outs,
        }
    }
}

pub trait AsDbInOut {
    type Primitive: AsDbPrimitive;
    fn db_inout(&self) -> (Option<Self::Primitive>, Option<Self::Primitive>);
}

pub trait AsDbMap {
    type InOut: AsDbInOut;
    fn db_ident(&self) -> String;
    fn db_content(&self) -> String;
    fn db_inout_pairs(&self) -> Vec<Self::InOut>;
}

impl TypeDb {
    pub(crate) fn get_maps(&self) -> DbResult<Vec<DbMap>> {
        let mut stmt = self.conn.prepare("SELECT uuid FROM maps")?;
        let uuids = stmt.query_map([], |row| row.get(0))?;
        let mut maps = Vec::new();
        for uuid in uuids {
            let uuid: String = uuid?;
            maps.push(self.get_map_from_uuid(uuid)?);
        }
        Ok(maps)
    }

    pub(crate) fn get_map_from_uuid(&self, uuid: String) -> DbResult<DbMap> {
        let mut stmt = self.conn.prepare("SELECT ident, content FROM maps WHERE uuid = ?")?;
        let (ident, content): (String, String) = stmt.query_row([&uuid], |row| {
            let ident: String = row.get(0)?;
            let content: String = row.get(1)?;
            Ok((ident, content))
        })?;
        let in_outs = self.get_in_outs_from_uuid(&uuid)?;
        Ok(DbMap {
            uuid,
            ident,
            content,
            in_outs,
        })
    }

    fn get_in_outs_from_uuid(&self, uuid: &str) -> DbResult<Vec<DbInOut>> {
        let mut stmt = self.conn.prepare(
            "SELECT pos, input_uuid, output_uuid FROM in_outs WHERE map_uuid = ? ORDER BY pos",
        )?;
        let in_outs = stmt.query_map([&uuid], |row| {
            let pos: i64 = row.get(0)?;
            let input_uuid: Option<String> = row.get(1)?;
            let output_uuid: Option<String> = row.get(2)?;
            Ok((pos, input_uuid, output_uuid))
        })?;
        let in_outs: Vec<_> = in_outs
            .enumerate()
            .map(|(i, x)| {
                let (pos, input_uuid, output_uuid) = x?;
                assert_eq!(i as i64, pos, "malformed db: in_outs.pos is not sorted");
                Ok((input_uuid, output_uuid))
            })
            .collect::<DbResult<Vec<_>>>()?;
        let in_outs: Vec<_> = in_outs
            .into_iter()
            .map(|(input_uuid, output_uuid)| {
                // todo! ?should get_*_from_uuid be infallible
                let input = input_uuid
                    .map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?
                    .flatten();
                let output = output_uuid
                    .map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?
                    .flatten();
                Ok((input, output))
            })
            .collect::<DbResult<Vec<_>>>()?;
        Ok(in_outs
            .into_iter()
            .map(|(input, output)| DbInOut { input, output })
            .collect())
        // todo!()
    }

    // todo! put all your code into 1 function with this neat trick doctors don't want you to know
    pub fn get_or_insert_map<M: AsDbMap, T: AsDbTag>(&self, map: &M, tags: &Vec<T>) -> DbResult<DbMap> {
        let ident = map.db_ident();
        let content = map.db_content();
        dbg!(&content);
        let in_outs = self.get_or_insert_in_outs(map.db_inout_pairs())?;
        dbg!();
        let mut statement = self.conn.prepare("SELECT uuid FROM maps WHERE ident = ?")?;
        let uuids = statement.query_map([&content], |row| row.get::<_, String>(0))?;
        // todo! `filter`
        let uuids: Vec<String> = uuids.collect::<Result<_, _>>()?;
        dbg!(&uuids);
        let mut maps: Vec<DbMap> = uuids
            .into_iter()
            .map(|uuid| {
                // get the position, input_uuid, and output_uuid but sort by position
                dbg!(&uuid);
                let mut statement = self.conn.prepare(
                "SELECT pos, input_uuid, output_uuid FROM in_outs WHERE map_uuid = ? ORDER BY pos",
            )?;
                let in_outs = statement.query_map([&uuid], |row| {
                    let pos: i64 = row.get(0)?;
                    dbg!(&pos);
                    let input_uuid: Option<String> = row.get(1)?;
                    dbg!(&input_uuid);
                    let output_uuid: Option<String> = row.get(2)?;
                    dbg!(&output_uuid);
                    Ok((pos, input_uuid, output_uuid))
                })?;
                let in_outs: Vec<_> = in_outs
                    .enumerate()
                    .map(|(i, x)| {
                        dbg!(&i, &x);
                        let (pos, input_uuid, output_uuid) = x?;
                        dbg!(&pos, &input_uuid, &output_uuid);
                        assert_eq!(i as i64, pos, "malformed db: in_outs.pos is not sorted");
                        Ok((input_uuid, output_uuid))
                    })
                    .collect::<DbResult<Vec<_>>>()?;
                let in_outs: Vec<_> = in_outs
                    .into_iter()
                    .map(|(input, output)| {
                        let input: Option<DbPrimitive> = input
                            .map(|uuid| self.get_primitive_from_uuid(uuid))
                            .transpose()?
                            .flatten(); // todo! ?unhandled error
                        let output: Option<DbPrimitive> = output
                            .map(|uuid| self.get_primitive_from_uuid(uuid))
                            .transpose()?
                            .flatten(); // todo! ?unhandled error
                                        // Ok((input, output))
                        Ok(DbInOut { input, output })
                    })
                    .collect::<DbResult<_>>()?;
                dbg!();

                let map = DbMap {
                    uuid,
                    ident: ident.clone(),
                    content: content.clone(),
                    in_outs,
                };
                Ok(map)
            })
            .collect::<DbResult<_>>()?;
        dbg!(&maps);
        maps.retain(|map| map.in_outs == in_outs);

        // todo! crashes on fatal error, db malformed
        assert!(
            maps.len() <= 1,
            "more than one map with the same ident and in_outs"
        );
        // todo! unwrap_or*
        let map = if let Some(map) = maps.into_iter().next() {
            map
        } else {
            let map = DbMap::new(ident.clone(), content.clone(), in_outs);
            self.insert_map(&map)?;
            map
        };
        self.tag_map(&map, tags)?;
        Ok(map)
    }

    fn get_or_insert_in_outs<InOut: AsDbInOut>(
        &self,
        in_outs: Vec<InOut>,
    ) -> DbResult<Vec<DbInOut>> {
        dbg!();
        in_outs
            .into_iter()
            .map(|in_out| in_out.db_inout())
            .map(|in_out| {
                dbg!();
                let input = in_out
                    .0
                    .map(|input| self.get_or_insert_primitive(&input))
                    .transpose()?;
                let output = in_out
                    .1
                    .map(|output| self.get_or_insert_primitive(&output))
                    .transpose()?;
                // todo!()
                // let input = input.map(|input| self.get_or_insert_primitive(&input)).transpose()?;
                // let output = output.map(|output| self.get_or_insert_primitive(&output)).transpose()?;
                Ok(DbInOut { input, output })
            })
            .collect::<DbResult<_>>()
        // todo!()
    }

    pub(crate) fn insert_map(&self, map: &DbMap) -> DbResult<()> {
        let mut statement = self
            .conn
            .prepare("INSERT INTO maps (uuid, ident) VALUES (?, ?)")?;
        statement.execute([map.uuid.clone(), map.content.clone()])?;
        let mut statement = self.conn.prepare(
            "INSERT INTO in_outs (map_uuid, pos, input_uuid, output_uuid) VALUES (?, ?, ?, ?)",
        )?;
        for (i, in_out) in map.in_outs.iter().enumerate() {
            dbg!();
            let input_uuid = in_out.input.as_ref().map(|x| x.uuid.clone());
            let output_uuid = in_out.output.as_ref().map(|x| x.uuid.clone());
            // let input_uuid = input.as_ref().map(|x| x.uuid.clone());
            // let output_uuid = output.as_ref().map(|x| x.uuid.clone());
            statement.execute(rusqlite::params![
                &map.uuid,
                i as i64,
                input_uuid,
                output_uuid
            ])?;
        }
        Ok(())
    }
}
