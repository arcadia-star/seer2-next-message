use std::fmt::Debug;
use bytes::Bytes;

use serde::Serialize;

pub use command::MessageCommand;
pub use message::Message;
pub use serde_bytes::SerdeError;
pub use parser::parse_data;

mod command;
mod encrypt;
mod message;
mod serde_bytes;
mod parser;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MessageSource {
    Client,
    Server,
}

pub trait MessageTrait: Debug {
    fn command(&self) -> MessageCommand;
    fn source(&self) -> MessageSource;
}

pub(crate) trait MessageParserTrait {
    fn command(&self) -> MessageCommand;
    fn source(&self) -> MessageSource;
    fn parse(&self, data: &Bytes) -> Result<String, SerdeError>;
}
