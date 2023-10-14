use std::io::Cursor;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};

use super::{encrypt, serde_bytes, SerdeError};
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

    fn need_encrypt(cid: i16) -> bool {
        cid < MessageCommand::LoginGetVerifyCode.cid() || cid > MessageCommand::UserLoginDeputize.cid()
    }
}
