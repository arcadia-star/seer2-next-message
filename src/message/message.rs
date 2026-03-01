use crate::message::SerdeError;
use bytes::{BufMut, Bytes, BytesMut};
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Message {
    pub len: u32,
    pub cid: u16,
    pub uid: u32,
    pub seq: u32,
    pub code: u32,
    pub body: Bytes,
}
impl Message {
    pub fn next_sequence(seq: u32, cid: u16, len: u32) -> u32 {
        let cid = cid as u32;
        cid % 13 + len % 21 + seq + 147 - seq / 7
    }
    pub fn update_len(&mut self) {
        self.len = 18 + self.body.len() as u32;
    }
    pub fn sign_code(&mut self) {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.put_u32_le(self.len);
        bytes_mut.put_u16_le(self.cid);
        bytes_mut.put_u32_le(self.uid);
        bytes_mut.put_u32_le(self.seq);
        bytes_mut.put_u32_le(0);
        bytes_mut.put_slice(&self.body);
        let code = bytes_mut
            .iter()
            .map(|e| *e as u32)
            .reduce(|a, b| a.overflowing_add(b).0)
            .unwrap();
        self.code = code % 100000;
    }
}

pub trait Body: Any + Debug + Send + Sync {
    fn command() -> crate::message::Command
    where
        Self: Sized;
    fn as_any_ref(&self) -> &dyn Any;
    fn from_bytes(bytes: &mut Bytes) -> Result<Self, SerdeError>
    where
        Self: Sized;
    fn to_bytes(&self) -> Result<Bytes, SerdeError>;
}
pub type Parser = fn(&mut Bytes) -> Result<Box<dyn Body>, SerdeError>;
