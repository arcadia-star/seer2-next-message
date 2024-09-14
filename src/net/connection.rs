use std::io::{Cursor, Error, ErrorKind};
use std::net::SocketAddr;

use bytes::{Buf, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const FLASH_POLICY_REQUEST: &str = "<policy-file-request/>\0";
const FLASH_POLICY_RESPONSE: &str = "<?xml version=\"1.0\"?><!DOCTYPE cross-domain-policy><cross-domain-policy><allow-access-from domain=\"*\" to-ports=\"*\" /></cross-domain-policy>\0";

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    addr: SocketAddr,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> Connection {
        Connection {
            stream,
            addr,
            buffer: BytesMut::with_capacity(4096),
        }
    }
    pub fn addr(&self) -> String {
        self.addr.to_string()
    }
    pub async fn read_target(&mut self, target: &str) -> Result<bool, Error> {
        let size = self.read_buffer().await?;
        if size > 0 {
            let bytes = target.as_bytes();
            let receive = &self.buffer[..size.min(bytes.len())];
            if bytes == receive {
                self.buffer.advance(bytes.len());
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub async fn read_frame(&mut self) -> Result<Option<Bytes>, Error> {
        loop {
            let (closed, data) = self.try_read_frame().await?;
            if closed {
                return Ok(None);
            }
            match data {
                Some(data) => {
                    return Ok(Some(data));
                }
                None => {}
            }
        }
    }
    pub async fn try_read_frame(&mut self) -> Result<(bool, Option<Bytes>), Error> {
        if let Some(frame) = self.parse_frame().await? {
            return Ok((false, Some(frame)));
        }

        if 0 == self.read_buffer().await? {
            return if self.buffer.is_empty() {
                Ok((true, None))
            } else {
                Err(Error::new(ErrorKind::UnexpectedEof, format!("connection reset by peer, remaining {:?}", self.buffer.remaining())))
            };
        }

        Ok((false, self.parse_frame().await?))
    }
    async fn parse_frame(&mut self) -> Result<Option<Bytes>, Error> {
        let mut buf = Cursor::new(&self.buffer[..]);
        if Self::check_frame(&mut buf).await? {
            let len = buf.position() as usize;
            buf.set_position(0);
            let bytes = Bytes::copy_from_slice(&buf.chunk()[..len]);
            self.buffer.advance(len);
            return Ok(Some(bytes));
        }
        Ok(None)
    }
    async fn check_frame(buf: &mut Cursor<&[u8]>) -> Result<bool, Error> {
        if buf.remaining() < 4 {
            return Ok(false);
        }
        let len = buf.get_u32_le() as usize;
        if len <= 4 {
            return Err(Error::new(ErrorKind::UnexpectedEof, format!("wrong len:{}", len)));
        }
        buf.set_position(0);
        if len > 1024 * 1024 {
            Err(Error::new(ErrorKind::UnexpectedEof, format!("wrong len:{}", len)))
        } else if len > buf.remaining() {
            Ok(false)
        } else {
            buf.advance(len);
            Ok(true)
        }
    }
    async fn read_buffer(&mut self) -> Result<usize, Error> {
        self.stream.read_buf(&mut self.buffer).await
    }
    pub async fn write(&mut self, src: &[u8]) -> Result<usize, Error> {
        self.stream.write(src).await
    }
    pub async fn discard_flash_policy(&mut self) -> Result<bool, Error> {
        let need_policy = self.read_target(FLASH_POLICY_REQUEST).await?;
        if need_policy {
            self.write(FLASH_POLICY_RESPONSE.as_bytes()).await?;
        }
        Ok(need_policy)
    }
}
