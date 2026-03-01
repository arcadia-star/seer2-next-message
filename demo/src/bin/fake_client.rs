#![allow(unused)]
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use message::entity::{
    server_parser, ItemGetListReq, LoginGetSessionReq, LoginGetSessionRsp, UserActivityCountReq,
    UserLoginOnlineReq,
};
use message::message::{Body, Command, Message, Parser};
use message::net::MessageCodec;
use message::utils::{CString, Hex};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Error;
use std::ops::Deref;
use std::process::exit;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

struct Config {
    uid: u32,
    password: String,
}
static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv::from_filename(".secret").ok();
    let map: HashMap<String, String> = dotenv::vars().collect();
    Config {
        uid: map.get("UID").unwrap().parse().unwrap(),
        password: map.get("PASSWORD").unwrap().clone(),
    }
});
static SERVER_PARSER: Lazy<HashMap<u16, Parser>> =
    Lazy::new(|| server_parser().into_iter().collect());

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut user = LoginUser::login(CONFIG.uid, &CONFIG.password).await?;
    user.send(ItemGetListReq {
        start: 1,
        end: 905999,
    })
    .await?;
    user.receive_until(Command::ItemGetList.cid()).await;
    user.send(UserActivityCountReq {
        activity_ids: (6000..9000).collect(),
    })
    .await?;
    user.receive_until(Command::UserActivityCount.cid()).await;
    Ok(())
}
type MessageFrame = Framed<TcpStream, MessageCodec>;
struct LoginUser {
    uid: u32,
    last_seq: u32,
    framed: MessageFrame,
}
impl LoginUser {
    async fn login(uid: u32, password: &str) -> Result<Self, Error> {
        let session = Self::get_session(uid, password).await?;
        let mut framed = Self::connect("118.89.149.189:1201").await?;
        let msg = build_message(
            uid,
            UserLoginOnlineReq {
                from_game: 10,
                session,
                top_left_tm_cid: CString::new("0"),
            },
            0,
        );
        let last_seq = msg.seq;
        framed.send(msg).await?;
        framed.next().await.unwrap()?;
        Ok(Self {
            uid,
            last_seq,
            framed,
        })
    }
    async fn send<T: Body>(&mut self, body: T) -> Result<(), Error> {
        let msg = build_message(self.uid, body, self.last_seq);
        self.last_seq = msg.seq;
        self.framed.send(msg).await?;
        Ok(())
    }
    async fn get_session(uid: u32, password: &str) -> Result<Hex<16>, Error> {
        let mut server = Self::connect("118.89.150.23:1863").await?;
        let mut password: Hex<32> = Hex::new(Bytes::from(format!(
            "{:x}",
            md5::compute(format!("{:x}", md5::compute(password)))
        )));
        println!("{}", password);
        let req = LoginGetSessionReq {
            password,
            revise_tm_cid: 65,
            product_id: 10,
            zero: 0,
            verify_img_id: Hex::new(Bytes::new()),
            verify_code: Hex::new(Bytes::new()),
            top_left_tm_cid: CString::new("0"),
        };
        let msg = build_message(uid, req, 0);
        server.send(msg).await?;
        let msg = server.next().await.unwrap()?;
        process_message(&msg);
        let rsp = LoginGetSessionRsp::from_bytes(&mut msg.body.clone());
        match rsp {
            Ok(rsp) => Ok(rsp.session.clone()),
            Err(_) => {
                eprintln!("login error");
                exit(1);
            }
        }
    }
    async fn connect(addr: &str) -> Result<MessageFrame, Error> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Framed::new(stream, MessageCodec))
    }
    async fn receive_until(&mut self, cid: u16) -> Message {
        loop {
            let msg = self.framed.next().await.unwrap().unwrap();
            process_message(&msg);
            if msg.cid == cid {
                return msg;
            }
        }
    }
}

fn build_message<T: Body>(uid: u32, body: T, last_seq: u32) -> Message {
    let mut message = Message {
        len: 0,
        cid: T::command().cid(),
        uid,
        seq: 0,
        code: 0,
        body: body.to_bytes().unwrap(),
    };
    message.update_len();
    message.seq = Message::next_sequence(last_seq, message.cid, message.len);
    message.sign_code();
    message
}

fn process_message(msg: &Message) {
    match SERVER_PARSER.deref().get(&msg.cid) {
        Some(parser) => {
            let mut bytes = msg.body.clone();
            match parser(&mut bytes) {
                Ok(body) => {
                    println!(
                        "len:{} cid:{} uid:{} seq:{} code:{} body:{:?}",
                        msg.len, msg.cid, msg.uid, msg.seq, msg.code, body
                    );
                }
                Err(err) => {
                    eprintln!(
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
            println!(
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
