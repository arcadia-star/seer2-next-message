use std::fmt::Debug;

use serde::Serialize;

pub use command::MessageCommand;
pub use message::Message;
pub use serde_bytes::SerdeError;

mod command;
mod encrypt;
mod message;
mod serde_bytes;

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
    fn parse(&self, msg: &Message) -> Result<Box<dyn MessageTrait>, SerdeError>;
}
