use crate::{TypeDb, DbResult, primitive::{AsDbPrimitive, DbPrimitive}};

#[allow(dead_code)]
mod test;

#[derive(Clone, Debug, Eq)]
pub struct DbMap {
    pub(crate) uuid: String,
    pub(crate) content: String,
    pub(crate) in_outs: Vec<(Option<DbPrimitive>, Option<DbPrimitive>)>,
}

impl PartialEq for DbMap {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.in_outs == other.in_outs
    }
}

impl DbMap {
    pub(crate) fn new(ident: String, in_outs: Vec<(Option<DbPrimitive>, Option<DbPrimitive>)>) -> Self {
        Self {
            uuid: TypeDb::new_uuid(),
            content: ident,
            in_outs,
        }
    }
}

pub trait AsDbMap {
    type Primitive: AsDbPrimitive;
    fn db_content(&self) -> String;
    fn db_inout_pairs(&self) -> Vec<(Option<Self::Primitive>, Option<Self::Primitive>)>;
}

const CREATE_TABLE: &str = "
    CREATE TABLE maps (
    uuid TEXT PRIMARY KEY,
    ident TEXT NOT NULL         -- not a unique identifier
)";
const CREATE_JUNCTION: &str = "
    CREATE TABLE map_fields (
    map_uuid TEXT NOT NULL,
    pos INTEGER NOT NULL,
    input_uuid TEXT,            -- NULL if the field is ()/None
    output_uuid TEXT,           -- NULL if the field is ()/None
    FOREIGN KEY (map_uuid) REFERENCES maps (uuid),
    FOREIGN KEY (input_uuid) REFERENCES primitives (uuid),
    FOREIGN KEY (output_uuid) REFERENCES primitives (uuid),
    PRIMARY KEY (map_uuid, pos)
)";

const DROP_TABLE: &str = "DROP TABLE IF EXISTS maps";
const DROP_JUNCTION: &str = "DROP TABLE IF EXISTS map_fields";

impl TypeDb {
    pub(crate) fn create_new_map_tables(&self) -> DbResult<()> {
        self.drop_map_tables()?;
        self.conn.execute(CREATE_TABLE, [])?;
        self.conn.execute(CREATE_JUNCTION, [])?;
        Ok(())
    }

    pub(crate) fn drop_map_tables(&self) -> DbResult<()> {
        self.conn.execute(DROP_TABLE, [])?;
        self.conn.execute(DROP_JUNCTION, [])?;
        Ok(())
    }

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
        let mut stmt = self.conn.prepare("SELECT ident FROM maps WHERE uuid = ?")?;
        let ident: String = stmt.query_row([&uuid], |row| row.get(0))?;
        let in_outs = self.get_in_outs_from_uuid(&uuid)?;
        Ok(DbMap { uuid, content: ident, in_outs })
    }

    fn get_in_outs_from_uuid(&self, uuid: &str) -> DbResult<Vec<(Option<DbPrimitive>, Option<DbPrimitive>)>> {
        let mut stmt = self.conn.prepare("SELECT pos, input_uuid, output_uuid FROM map_fields WHERE map_uuid = ? ORDER BY pos")?;
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
                assert_eq!(i as i64, pos, "malformed db: map_fields.pos is not sorted");
                Ok((input_uuid, output_uuid))
            })
            .collect::<DbResult<Vec<_>>>()?;
        let in_outs: Vec<_> = in_outs
            .into_iter()
            .map(|(input_uuid, output_uuid)| {
                // todo! ?should get_*_from_uuid be infallible
                let input = input_uuid.map(|uuid| self.get_primitive_from_uuid(uuid)).transpose()?.flatten();
                let output = output_uuid.map(|uuid| self.get_primitive_from_uuid(uuid)).transpose()?.flatten();
                Ok((input, output))
            })
            .collect::<DbResult<Vec<_>>>()?;
        Ok(in_outs)
    }

    // todo! put all your code into 1 function with this neat trick doctors don't want you to know
    pub(crate) fn get_or_insert_map<M: AsDbMap>(&self, map: &M) -> DbResult<DbMap> {
        let content = map.db_content();
        dbg!(&content);
        let in_outs = self.get_or_insert_in_outs(map.db_inout_pairs())?;
        
        let mut statement = self.conn.prepare("SELECT uuid FROM maps WHERE ident = ?")?;
        let uuids = statement.query_map([&content], |row| row.get::<_, String>(0))?;
        // todo! `filter`
        let uuids: Vec<String> = uuids.collect::<Result<_, _>>()?;
        dbg!(&uuids);
        let mut maps: Vec<DbMap> = uuids.into_iter().map(|uuid| {
            // get the position, input_uuid, and output_uuid but sort by position
            dbg!(&uuid);
            let mut statement = self.conn.prepare(
                "SELECT pos, input_uuid, output_uuid FROM map_fields WHERE map_uuid = ? ORDER BY pos",
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
                    assert_eq!(i as i64, pos, "malformed db: map_fields.pos is not sorted");
                    Ok((input_uuid, output_uuid))
                })
                .collect::<DbResult<Vec<_>>>()?;
            let in_outs: Vec<_> = in_outs.into_iter().map(|(input, output)| {
                let input: Option<DbPrimitive> = input.map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?.flatten(); // todo! ?unhandled error
                let output: Option<DbPrimitive> = output.map(|uuid| self.get_primitive_from_uuid(uuid))
                    .transpose()?.flatten(); // todo! ?unhandled error
                Ok((input, output))
            }).collect::<DbResult<_>>()?;
            let map = DbMap {
                uuid,
                content: content.clone(),
                in_outs,
            };
            Ok(map)
        }).collect::<DbResult<_>>()?;
        dbg!(&maps);
        maps.retain(|map| map.in_outs == in_outs);
        
        // todo! crashes on fatal error, db malformed
        assert!(maps.len() <= 1, "more than one map with the same ident and in_outs");
        // todo! unwrap_or*
        Ok(if let Some(map) = maps.into_iter().next() {
            map
        } else {
            let map = DbMap::new(content.clone(), in_outs);
            self.insert_map(&map)?;
            map
        })
    }

    fn get_or_insert_in_outs<P: AsDbPrimitive>(&self, in_outs: Vec<(Option<P>, Option<P>)>) -> DbResult<Vec<(Option<DbPrimitive>, Option<DbPrimitive>)>> {
        in_outs.into_iter().map(|(input, output)| {
            let input = input.map(|input| self.get_or_insert_primitive(&input)).transpose()?;
            let output = output.map(|output| self.get_or_insert_primitive(&output)).transpose()?;
            Ok((input, output))
        }).collect::<DbResult<_>>()
    }
    
    pub(crate) fn insert_map(&self, map: &DbMap) -> DbResult<()> {
        let mut statement = self.conn.prepare("INSERT INTO maps (uuid, ident) VALUES (?, ?)")?;
        statement.execute([map.uuid.clone(), map.content.clone()])?;
        let mut statement = self.conn.prepare("INSERT INTO map_fields (map_uuid, pos, input_uuid, output_uuid) VALUES (?, ?, ?, ?)")?;
        for (i, (input, output)) in map.in_outs.iter().enumerate() {
            let input_uuid = input.as_ref().map(|x| x.uuid.clone());
            let output_uuid = output.as_ref().map(|x| x.uuid.clone());
            statement.execute(rusqlite::params![&map.uuid, i as i64, input_uuid, output_uuid])?;
        }
        Ok(())
    }
}