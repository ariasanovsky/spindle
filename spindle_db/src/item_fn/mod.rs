mod get_or_insert;
mod query;
mod tables;
#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbItemFn {
    pub uuid: String,
    pub ident: String,
    pub content: String,
}

pub trait AsDbItemFn {
    fn db_item_ident(&self) -> String;
    fn db_item_content(&self) -> String;
}
