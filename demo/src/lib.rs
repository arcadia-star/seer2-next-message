use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use message::entity::{
    client_parser, server_parser, LoginGetSessionReq, LoginGetSessionRsp, UserLoginOnlineReq,
    UserLoginOnlineRsp,
};
use message::message::{Body, Message, Parser};
use message::net::MessageCodec;
use message::utils::{CString, Hex};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::ops::Deref;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tracing::{error, info};

pub struct Client {
    framed: Framed<TcpStream, MessageCodec>,
    builder: MessageBuilder,
}

impl Client {
    pub async fn connect(addr: &str, uid: u32) -> Result<Client, Error> {
        Ok(Self {
            framed: connect(addr).await?,
            builder: MessageBuilder::new(uid),
        })
    }
    pub async fn send<T: Body, R: Body>(&mut self, body: T) -> Result<R, Error> {
        self.framed.send(self.builder.build(body)).await?;
        loop {
            match self.framed.next().await {
                None => {}
                Some(Err(e)) => return Err(e),
                Some(Ok(msg)) => {
                    if msg.cid == R::command().cid() {
                        let data = R::from_bytes(&mut msg.body.clone());
                        return match data {
                            Ok(data) => Ok(data),
                            Err(err) => Err(Error::new(ErrorKind::Other, err)),
                        };
                    }
                    let addr = self.framed.get_ref().peer_addr()?;
                    log_message(false, &addr, &msg);
                }
            }
        }
    }
    pub async fn session(&mut self, password: &str) -> Result<Hex<16>, Error> {
        let password: Hex<32> = Hex::new(Bytes::from(md5(&md5(password))));
        let rsp: LoginGetSessionRsp = self
            .send(LoginGetSessionReq {
                password,
                revise_tm_cid: 65,
                product_id: 10,
                zero: 0,
                verify_img_id: Hex::new(Bytes::new()),
                verify_code: Hex::new(Bytes::new()),
                top_left_tm_cid: CString::new("0"),
            })
            .await?;
        Ok(rsp.session)
    }
    pub async fn login(uid: u32, password: &str) -> Result<(Client, UserLoginOnlineRsp), Error> {
        let session = Client::connect("118.89.150.23:1863", uid)
            .await?
            .session(password)
            .await?;
        let mut client = Client::connect("118.89.149.189:1201", uid).await?;
        let rsp: UserLoginOnlineRsp = client
            .send(UserLoginOnlineReq {
                from_game: 10,
                session,
                top_left_tm_cid: CString::new("0"),
            })
            .await?;
        Ok((client, rsp))
    }
}

pub async fn connect(addr: &str) -> Result<Framed<TcpStream, MessageCodec>, Error> {
    Ok(Framed::new(TcpStream::connect(addr).await?, MessageCodec))
}

pub struct MessageBuilder {
    uid: u32,
    seq: u32,
}

impl MessageBuilder {
    pub fn new(uid: u32) -> Self {
        Self { uid, seq: 0 }
    }
    pub fn build<T: Body>(&mut self, body: T) -> Message {
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
        message
    }
}

static CLIENT_PARSER: Lazy<HashMap<u16, Parser>> =
    Lazy::new(|| client_parser().into_iter().collect());
static SERVER_PARSER: Lazy<HashMap<u16, Parser>> =
    Lazy::new(|| server_parser().into_iter().collect());
pub fn log_message(client: bool, addr: &SocketAddr, msg: &Message) {
    match (if client {
        CLIENT_PARSER.deref()
    } else {
        SERVER_PARSER.deref()
    })
    .get(&msg.cid)
    {
        Some(parser) => {
            let mut bytes = msg.body.clone();
            match parser(&mut bytes) {
                Ok(body) => {
                    info!(
                        ?addr,
                        "len:{} cid:{} uid:{} seq:{} code:{} body:{:?}",
                        msg.len,
                        msg.cid,
                        msg.uid,
                        msg.seq,
                        msg.code,
                        body
                    );
                }
                Err(err) => {
                    error!(
                        ?addr,
                        "len:{} cid:{} uid:{} seq:{} code:{} hex:{} error:{err}",
                        msg.len,
                        msg.cid,
                        msg.uid,
                        msg.seq,
                        msg.code,
                        hex::encode(&msg.body)
                    );
                }
            }
        }
        None => {
            info!(
                ?addr,
                "len:{} cid:{} uid:{} seq:{} code:{} hex:{}",
                msg.len,
                msg.cid,
                msg.uid,
                msg.seq,
                msg.code,
                hex::encode(&msg.body)
            );
        }
    }
}

fn md5(str: &str) -> String {
    format!("{:x}", md5::compute(str))
}

pub struct Secrets {
    pub uid: u32,
    pub password: String,
}

impl Secrets {
    pub fn from_env() -> Self {
        dotenv::from_filename(".secret").ok();
        let map: HashMap<String, String> = dotenv::vars().collect();
        Self {
            uid: map.get("UID").unwrap().parse().unwrap(),
            password: map.get("PASSWORD").unwrap().clone(),
        }
    }
}
