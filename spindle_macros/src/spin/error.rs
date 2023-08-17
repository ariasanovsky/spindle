#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl From<Error> for syn::Error {
    fn from(value: Error) -> Self {
        todo!("c")
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}