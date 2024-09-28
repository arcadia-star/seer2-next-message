use crate::error::Error;
use crate::message::{MessageCommand, MessageData};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct MessageBody(Result<Box<dyn MessageData>, Error>);
#[derive(Debug)]
pub struct Message {
    pub len: u32,
    pub cid: u16,
    pub uid: u32,
    pub seq: u32,
    pub code: u32,
    pub body: MessageBody,
}
pub type MessageParser = fn(&mut Bytes) -> Result<Box<dyn MessageData>, Error>;
static PARSERS: Lazy<(HashMap<u16, MessageParser>, HashMap<u16, MessageParser>)> = Lazy::new(|| {
    (
        crate::entity::client_parser().into_iter().collect(),
        crate::entity::server_parser().into_iter().collect(),
    )
});
impl MessageBody {
    pub fn to_bytes(&self) -> Bytes {
        match &self.0 {
            Ok(data) => { data.to_bytes().unwrap() }
            Err(err) => { err.to_bytes() }
        }
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.to_bytes())
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        match &self.0 {
            Ok(data) => { data.as_any_ref().downcast_ref() }
            Err(_) => { None }
        }
    }
    pub fn deref(&self) -> &Result<Box<dyn MessageData>, Error> {
        &self.0
    }
}
impl Debug for MessageBody {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{:?}", &self.0))
    }
}
impl Message {
    const HEAD_LEN: usize = 18;
    fn parse(parser: &HashMap<u16, MessageParser>, bytes: &mut Bytes) -> Message {
        let cid = Bytes::copy_from_slice(&bytes[4..6]).get_u16_le();
        let mut bytes = bytes;
        let mut bytes0;
        if MessageCommand::need_encrypt(cid) {
            bytes0 = Bytes::from(crate::message::encrypt::decrypt(&bytes));
            bytes = &mut bytes0;
        }
        let len = bytes.get_u32_le();
        let cid = bytes.get_u16_le();
        let uid = bytes.get_u32_le();
        let seq = bytes.get_u32_le();
        let code = bytes.get_u32_le();
        let body = MessageBody(match parser.get(&cid) {
            Some(parser) => parser(bytes),
            None => Err(Error::ParserNotExist(cid, bytes.clone())),
        });
        Message { len, cid, uid, seq, code, body }
    }
    pub fn parse_client(bytes: &mut Bytes) -> Message {
        Self::parse(&PARSERS.0, bytes)
    }
    pub fn parse_server(bytes: &mut Bytes) -> Message {
        Self::parse(&PARSERS.1, bytes)
    }
    fn to_bytes(&self, code: u32) -> Bytes {
        let mut data = self.body.to_bytes();
        let len = Self::HEAD_LEN + data.len();
        let mut bytes = BytesMut::with_capacity(len);
        bytes.put_u32_le(len as u32);
        bytes.put_u16_le(self.cid);
        bytes.put_u32_le(self.uid);
        bytes.put_u32_le(self.seq);
        bytes.put_u32_le(code);
        bytes.put(&mut data);
        bytes.freeze()
    }
    pub fn to_server_bytes(&self) -> Bytes {
        let mut bytes = self.to_bytes(self.code);
        if MessageCommand::need_encrypt(self.cid) {
            bytes = Bytes::from(crate::message::encrypt::encrypt(&bytes));
        }
        bytes
    }
    pub fn to_client_bytes(&self) -> Bytes {
        let bytes = self.to_bytes(0);
        let code = bytes.iter().map(|e| *e as u32)
            .reduce(|a, b| a.overflowing_add(b).0)
            .unwrap() % 100000;
        let mut bytes = self.to_bytes(code);
        if MessageCommand::need_encrypt(self.cid) {
            bytes = Bytes::from(crate::message::encrypt::encrypt(&bytes));
        }
        bytes
    }
    pub fn new<T: MessageData>(uid: u32, data: T) -> Self {
        Message {
            len: 0,
            cid: T::command(),
            uid,
            seq: 0,
            code: 0,
            body: MessageBody(Ok(Box::new(data))),
        }
    }
    pub fn response_error(&self, code: u32) -> Self {
        Message {
            len: 0,
            cid: self.cid,
            uid: self.uid,
            seq: self.seq,
            code,
            body: MessageBody(Ok(Box::new(Bytes::new()))),
        }
    }
    pub fn next_seq(&self, seq: u32) -> u32 {
        let data = self.body.to_bytes();
        let len = (Self::HEAD_LEN + data.len()) as u32;
        let cid = self.cid as u32;
        cid % 13 + len % 21 + seq + 147 - seq / 7
    }
}
