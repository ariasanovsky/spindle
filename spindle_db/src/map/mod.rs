use crate::{
    primitive::{AsDbPrimitive, DbPrimitive},
    tag::AsDbTag,
    DbResult, TypeDb,
};

#[cfg(test)]
mod test;

#[derive(Clone, Debug, Eq)]
pub struct DbMap {
    pub uuid: String,
    pub ident: String,
    pub content: String,
    pub in_outs: Vec<DbInOut>,
    pub range_type: Option<DbPrimitive>,
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
    pub(crate) fn new(
        ident: String,
        content: String,
        in_outs: Vec<DbInOut>,
        range_type: Option<DbPrimitive>,
    ) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            ident,
            content,
            in_outs,
            range_type,
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
    fn db_inouts(&self) -> Vec<Self::InOut>;
    fn range_type(&self) -> Option<String>;
}

impl TypeDb {
    // todo! refactor with collect
    pub fn get_maps(&self) -> DbResult<Vec<DbMap>> {
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
        let mut stmt = self
            .conn
            .prepare("SELECT ident, content, range_uuid FROM maps WHERE uuid = ?")?;
        let (ident, content, range_type): (String, String, Option<DbPrimitive>) =
            stmt.query_row([&uuid], |row| {
                let ident: String = row.get(0)?;
                let content: String = row.get(1)?;
                let range_uuid: Option<String> = row.get(2)?;
                let range_type = range_uuid.map(|range_uuid| self.get_primitive_from_uuid(range_uuid)).transpose()?;
                Ok((ident, content, range_type))
        })?;
        let in_outs = self.get_in_outs_from_uuid(&uuid)?;
        Ok(DbMap {
            uuid,
            ident,
            content,
            in_outs,
            range_type,
        })
    }

    fn get_in_outs_from_uuid(&self, uuid: &str) -> DbResult<Vec<DbInOut>> {
        // todo! seems way too verbose...
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
                    .transpose()?;
                let output = output_uuid
                    .map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?;
                Ok((input, output))
            })
            .collect::<DbResult<Vec<_>>>()?;
        Ok(in_outs
            .into_iter()
            .map(|(input, output)| DbInOut { input, output })
            .collect())
    }

    pub fn get_or_insert_map<M: AsDbMap, T: AsDbTag>(
        &self,
        map: &M,
        tags: &[T],
    ) -> DbResult<DbMap> {
        let db_map = self
            .get_map(map)
            .transpose()
            .unwrap_or_else(|| self.insert_map(map))?;
        self.tag_map(&db_map, tags)?;
        Ok(db_map)
    }

    pub(crate) fn get_map<M: AsDbMap>(&self, map: &M) -> DbResult<Option<DbMap>> {
        // get all maps with the same ident & contents
        let mut statement = self
            .conn
            .prepare("SELECT uuid FROM maps WHERE ident = ? AND content = ?")?;
        let mut maps = statement.query_map([&map.db_ident(), &map.db_content()], |row| {
            let uuid: String = row.get(0)?;
            self.get_map_from_uuid(uuid)
        })?;
        let first = maps.next().transpose()?;
        let second = maps.next().transpose()?;
        match (first, second) {
            (None, _) => Ok(None),
            (Some(db_map), None) => Ok(Some(db_map)),
            (Some(db_map_1), Some(db_map_2)) => {
                panic!("duplicate maps in db: {db_map_1:?} {db_map_2:?}")
            }
        }
    }

    pub(crate) fn insert_map<M: AsDbMap>(&self, map: &M) -> DbResult<DbMap> {
        // get in_out pairs from map
        let in_outs: Vec<DbInOut> = map
            .db_inouts()
            .into_iter()
            .map(|in_out| {
                let (input, output) = in_out.db_inout();
                let input = input
                    .map(|input| self.get_or_insert_primitive(&input))
                    .transpose()?;
                let output = output
                    .map(|output| self.get_or_insert_primitive(&output))
                    .transpose()?;
                Ok(DbInOut { input, output })
            })
            .collect::<DbResult<_>>()?;
        let range_type = map.range_type().map(|range_type| self.get_or_insert_primitive(&range_type)).transpose()?;
        let map = DbMap::new(
            map.db_ident(),
            map.db_content(),
            in_outs,
            range_type,
        );
        self.insert_db_map(&map)?;
        Ok(map)
    }

    fn insert_db_map(&self, map: &DbMap) -> DbResult<()> {
        let mut statement = self
            .conn
            .prepare("INSERT INTO maps (uuid, ident, content, range_uuid) VALUES (?, ?, ?, ?)")?;
        let DbMap {
            uuid,
            ident,
            content,
            in_outs,
            range_type,
        } = &map;
        let range_uuid = range_type.as_ref().map(|x| &x.uuid);
        let _: usize =
            statement.execute(rusqlite::params![uuid, ident, content, range_uuid])?;
        let mut statement = self.conn.prepare(
            "INSERT INTO in_outs (map_uuid, pos, input_uuid, output_uuid) VALUES (?, ?, ?, ?)",
        )?;
        for (i, in_out) in in_outs.iter().enumerate() {
            let input_uuid = in_out.input.as_ref().map(|x| &x.uuid);
            let output_uuid = in_out.output.as_ref().map(|x| &x.uuid);
            let _: usize =
                statement.execute(rusqlite::params![uuid, i, input_uuid, output_uuid])?;
        }
        Ok(())
    }
}
