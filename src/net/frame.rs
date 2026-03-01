use crate::message::{decrypt, encrypt, Command, Message};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io::{Cursor, Error as IoError};
use tokio_util::codec::{Decoder, Encoder};

pub struct MessageCodec;
impl Decoder for MessageCodec {
    type Item = Message;
    type Error = IoError;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // Not enough data
            return Ok(None);
        }
        let len = Cursor::new(&src).get_u32_le() as usize;
        if src.len() < len {
            // Not enough data
            return Ok(None);
        }
        let mut bytes = Cursor::new(&src).copy_to_bytes(len);
        let cid = Cursor::new(&bytes[4..]).get_u16_le();
        if Command::has_encrypt(cid) {
            bytes = Bytes::from(decrypt(&bytes));
        }
        let msg = Message {
            len: bytes.get_u32_le(),
            cid: bytes.get_u16_le(),
            uid: bytes.get_u32_le(),
            seq: bytes.get_u32_le(),
            code: bytes.get_u32_le(),
            body: bytes,
        };
        src.advance(len);
        Ok(Some(msg))
    }
}
impl Encoder<Message> for MessageCodec {
    type Error = IoError;
    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let mut bytes_mut = BytesMut::new();
        bytes_mut.put_u32_le(item.len);
        bytes_mut.put_u16_le(item.cid);
        bytes_mut.put_u32_le(item.uid);
        bytes_mut.put_u32_le(item.seq);
        bytes_mut.put_u32_le(item.code);
        bytes_mut.extend_from_slice(&item.body);
        let mut bytes = bytes_mut.freeze();
        if Command::has_encrypt(item.cid) {
            bytes = Bytes::from(encrypt(&bytes));
        }
        dst.extend_from_slice(&bytes);
        Ok(())
    }
}
const FLASH_POLICY_REQUEST: &str = "<policy-file-request/>\0";
const FLASH_POLICY_RESPONSE: &str = "<?xml version=\"1.0\"?><!DOCTYPE cross-domain-policy><cross-domain-policy><allow-access-from domain=\"*\" to-ports=\"*\" /></cross-domain-policy>\0";
pub struct FlashPolicyCodec;
impl Decoder for FlashPolicyCodec {
    type Item = bool;
    type Error = IoError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let flash_bytes = FLASH_POLICY_REQUEST.as_bytes();
        let mut src = Cursor::new(&mut *src);
        for target in flash_bytes {
            if !src.has_remaining() {
                return Ok(None);
            }
            if src.get_u8() != *target {
                return Ok(Some(false));
            }
        }
        Ok(Some(true))
    }
}
impl Encoder<bool> for FlashPolicyCodec {
    type Error = IoError;
    fn encode(&mut self, item: bool, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if item {
            dst.extend_from_slice(FLASH_POLICY_RESPONSE.as_bytes());
        }
        Ok(())
    }
}
