#[derive(Debug)]
pub enum Error {

}

impl From<Error> for syn::Error {
    fn from(value: Error) -> Self {
        todo!()
    }
}
