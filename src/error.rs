use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum Error {
    ParserNotExist(u16, Bytes),
    ParseError(String, Bytes),
    SerdeError(String),
}

impl Error {
    pub fn to_bytes(&self) -> Bytes {
        match self {
            Error::ParserNotExist(_, data) => { data.clone() }
            Error::ParseError(_, data) => { data.clone() }
            _ => unreachable!()
        }
    }
}