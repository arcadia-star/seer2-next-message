use std::io::Cursor;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use super::{encrypt, MessageTrait, serde_bytes, SerdeError};
use super::command::MessageCommand;

#[derive(Debug)]
pub struct Message {
    pub len: i32,
    pub cid: i16,
    pub uid: i32,
    pub seq: i32,
    pub code: i32,
    pub data: Bytes,
}

impl Message {
    pub const SUCCESS: i32 = 0;
    const HEAD_LEN: usize = 18;
    pub fn new(mut bytes: Bytes) -> Self {
        let mut cursor = Cursor::new(&bytes);
        cursor.get_i32_le();
        let cid = cursor.get_i16_le();
        if Self::need_encrypt(cid) {
            bytes = Bytes::from(encrypt::decrypt(bytes.chunk()));
        }
        let len = bytes.get_i32_le();
        let cid = bytes.get_i16_le();
        let uid = bytes.get_i32_le();
        let seq = bytes.get_i32_le();
        let code = bytes.get_i32_le();
        let data = bytes;
        Self { len, cid, uid, seq, code, data }
    }
    pub fn bytes(&self) -> Bytes {
        let len = Self::HEAD_LEN + self.data.len();
        let mut bytes = BytesMut::with_capacity(len);
        bytes.put_i32_le(len as i32);
        bytes.put_i16_le(self.cid);
        bytes.put_i32_le(self.uid);
        bytes.put_i32_le(self.seq);
        bytes.put_i32_le(self.code);
        bytes.put(&mut self.data.clone());
        if Self::need_encrypt(self.cid) {
            Bytes::from(encrypt::encrypt(bytes.chunk()))
        } else {
            bytes.freeze()
        }
    }

    pub fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a mut Bytes) -> Result<T, SerdeError> {
        serde_bytes::from_bytes(bytes)
    }
    pub fn serialize<T: Serialize>(data: &T) -> Result<Bytes, SerdeError> {
        serde_bytes::to_bytes(data)
    }

    pub fn new4client<T: MessageTrait + Serialize>(uid: i32, last_seq: i32, data: &T) -> Result<Message, SerdeError> {
        let data_bytes = Self::serialize(data)?;
        let cid = data.command().cid();
        let len = (Self::HEAD_LEN + data_bytes.len()) as i32;
        let seq = Self::next_seq(last_seq, cid as i32, len);
        let mut m = Message {
            len,
            cid,
            uid,
            seq,
            code: 0,
            data: data_bytes,
        };
        m.sign_code();
        Ok(m)
    }

    fn next_seq(last_seq: i32, cid: i32, len: i32) -> i32 {
        cid % 13 + len % 21 + last_seq + 147 - last_seq / 7
    }
    fn sign_code(&mut self) {
        self.code = 0;
        let adder = |a: i32, b| a.overflowing_add(b).0;
        let a = self.bytes().iter().map(|e| *e as i32).reduce(adder).unwrap_or(0);
        self.code = a % 100000;
    }
    fn need_encrypt(cid: i16) -> bool {
        cid < MessageCommand::LoginGetVerifyCode.cid() || cid > MessageCommand::UserLoginDeputize.cid()
    }
}
