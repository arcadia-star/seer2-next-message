use crate::error::Error;
use bytes::{Buf, Bytes};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait MessageData: Debug + Send + Sync + Any {
    fn command() -> u16
    where
        Self: Sized;
    fn from_bytes(bytes: &mut Bytes) -> Result<Self, Error>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Bytes, Error>;
    fn to_json(&self) -> Result<String, Error>;
    fn as_any_ref(&self) -> &dyn Any;
}
impl<T: MessageData> MessageData for Arc<T> {
    fn command() -> u16
    where
        Self: Sized,
    {
        T::command()
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Arc::new(T::from_bytes(bytes)?))
    }

    fn to_bytes(&self) -> Result<Bytes, Error> {
        (&**self).to_bytes()
    }

    fn to_json(&self) -> Result<String, Error> {
        (&**self).to_json()
    }

    fn as_any_ref(&self) -> &dyn Any {
        &**self
    }
}
impl MessageData for Bytes {
    fn command() -> u16
    where
        Self: Sized,
    {
        0
    }

    fn from_bytes(bytes: &mut Bytes) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(bytes.clone())
    }

    fn to_bytes(&self) -> Result<Bytes, Error> {
        Ok(self.clone())
    }

    fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(self.chunk()).map_err(|err| Error::SerdeError(err.to_string()))
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}