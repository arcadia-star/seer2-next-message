use crate::PrintMessage;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use message::entity::{
    LoginGetSessionReq, LoginGetSessionRsp, UserLoginOnlineReq, UserLoginOnlineRsp,
};
use message::message::{Body, Message};
use message::net::MessageCodec;
use message::utils::{CString, Hex};
use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use tokio::net::TcpStream;
use tokio::task::yield_now;
use tokio_socks::tcp::Socks5Stream;
use tokio_util::codec::Framed;

pub struct ClientOptions {
    uid: u32,
    password: Hex<32>,
    pub login_addr: String,
    pub game_addr: String,
    pub ignore_cid: HashSet<u16>,
    proxy: Option<String>,
}

impl ClientOptions {
    pub fn new(uid: u32, password: &str) -> Self {
        fn md5(str: &str) -> String {
            format!("{:x}", md5::compute(str))
        }
        let password = Hex::new_copy(md5(&md5(password)).as_bytes());
        Self {
            uid,
            password,
            login_addr: "118.89.150.23:1863".to_string(),
            game_addr: "118.89.149.189:1243".to_string(),
            ignore_cid: vec![].into_iter().collect(),
            proxy: None,
        }
    }
    pub fn proxy(mut self, proxy: &str) -> Self {
        self.proxy = Some(proxy.to_string());
        self
    }
    pub fn build(self) -> Client {
        Client { options: self }
    }
}

pub struct Client {
    options: ClientOptions,
}

impl Client {
    async fn connect(&self, addr: &str) -> Result<ClientConnection, Error> {
        let mut conn = match &self.options.proxy {
            Some(proxy) => {
                ClientConnection::connect_with_proxy(proxy, addr, self.options.uid).await?
            }
            None => ClientConnection::connect(addr, self.options.uid).await?,
        };
        conn.ignore_cid = self.options.ignore_cid.clone();
        Ok(conn)
    }
    pub async fn login(&mut self) -> Result<(ClientConnection, UserLoginOnlineRsp), Error> {
        let mut conn = self.connect(&self.options.login_addr).await?;
        let rsp: LoginGetSessionRsp = conn
            .send_then_wait(LoginGetSessionReq {
                password: self.options.password.clone(),
                revise_tm_cid: 65,
                product_id: 10,
                zero: 0,
                verify_img_id: Hex::new(Bytes::new()),
                verify_code: Hex::new(Bytes::new()),
                top_left_tm_cid: CString::new("0"),
            })
            .await?;
        let session = rsp.session;
        // println!("session {}", session);
        let mut conn = self.connect(&self.options.game_addr).await?;
        let rsp = conn
            .send_then_wait(UserLoginOnlineReq {
                from_game: 10,
                session,
                top_left_tm_cid: CString::new("0"),
            })
            .await?;
        Ok((conn, rsp))
    }
}

pub struct ClientConnection {
    uid: u32,
    seq: u32,
    framed: Framed<TcpStream, MessageCodec>,
    ignore_cid: HashSet<u16>,
}

impl ClientConnection {
    pub async fn connect_with_proxy(
        proxy: &str,
        addr: &str,
        uid: u32,
    ) -> Result<ClientConnection, Error> {
        Ok(Self {
            uid,
            seq: 0,
            framed: connect_with_proxy(proxy, addr).await?,
            ignore_cid: Default::default(),
        })
    }
    pub async fn connect(addr: &str, uid: u32) -> Result<ClientConnection, Error> {
        Ok(Self {
            uid,
            seq: 0,
            framed: connect(addr).await?,
            ignore_cid: Default::default(),
        })
    }
    pub async fn send<T: Body>(&mut self, body: T) -> Result<(), Error> {
        let body = body.to_bytes().unwrap();
        let cid = T::command().cid();
        let len = Message::HEADER_LEN + body.len() as u32;
        self.seq = Message::next_sequence(self.seq, cid, len);
        let mut message = Message {
            len,
            cid,
            uid: self.uid,
            seq: self.seq,
            code: 0,
            body,
        };
        message.sign_code();
        self.framed.send(message).await
    }
    pub async fn wait(&mut self, cid: u16) -> Result<Message, Error> {
        loop {
            match self.framed.next().await {
                None => return Err(ErrorKind::ConnectionAborted.into()),
                Some(Err(e)) => return Err(e),
                Some(Ok(msg)) => {
                    if msg.cid == cid {
                        return Ok(msg);
                    }
                    if !self.ignore_cid.contains(&msg.cid) {
                        println!("{}", msg.data_msg(false));
                    }
                }
            }
            yield_now().await;
        }
    }
    pub async fn send_then_wait_raw<T: Body>(&mut self, body: T) -> Result<Message, Error> {
        self.send(body).await?;
        self.wait(T::command().cid()).await
    }
    pub async fn send_then_wait<T: Body, R: Body>(&mut self, body: T) -> Result<R, Error> {
        let mut message = self.send_then_wait_raw(body).await?;
        R::from_bytes(&mut message.body).map_err(|e| Error::new(ErrorKind::Other, e))
    }
}

pub async fn connect(addr: &str) -> Result<Framed<TcpStream, MessageCodec>, Error> {
    Ok(Framed::new(TcpStream::connect(addr).await?, MessageCodec))
}

pub async fn connect_with_proxy(
    proxy: &str,
    addr: &str,
) -> Result<Framed<TcpStream, MessageCodec>, Error> {
    Ok(Framed::new(
        Socks5Stream::connect(proxy, addr)
            .await
            .map_err(|e| Error::new(ErrorKind::Other, e))?
            .into_inner(),
        MessageCodec,
    ))
}
