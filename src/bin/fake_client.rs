use std::io::{Error, Write};
use std::process::exit;

use bytes::{Buf, BufMut};
use serde::Serialize;
use tokio::net::TcpSocket;

use message::entity::{LoginGetSessionReq, LoginGetSessionRsp, UserEnterMapReq, UserLevelMapReq, UserLoginOnlineReq};
use message::message::{Message, MessageSource, parse_data};
use message::message::MessageTrait;
use message::net::Connection;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut user = LoginUser::login(12345678, "12345678").await?;
    user.send(UserLevelMapReq {}).await?;
    user.send(UserEnterMapReq { map: 90, scene: 0, loc_x: 529, loc_y: 427, behavior: 0 }).await?;
    loop {
        if !user.receive().await? {
            break;
        }
    }
    Ok(())
}

struct LoginUser {
    uid: i32,
    last_seq: i32,
    server: Connection,
}

impl LoginUser {
    async fn login(uid: i32, password: &'static str) -> Result<Self, Error> {
        let session = Self::get_session(uid, password).await?;
        println!("session: {:?}", session);
        let server = Self::connect("118.89.149.189:1201").await?;
        let mut user = Self {
            uid,
            last_seq: 0,
            server,
        };
        user.send(UserLoginOnlineReq {
            from_game: 10,
            session,
            top_left_tm_cid: [48, 0, 0, 0, 0, 0, 0, 0],
        }).await?;
        Ok(user)
    }
    async fn send(&mut self, data: impl MessageTrait + Serialize) -> Result<(), Error> {
        let m = Message::new4client(self.uid, self.last_seq, &data).unwrap();
        self.last_seq = m.seq;
        println!("req: {:?}", m);
        self.server.write(m.bytes().chunk()).await?;
        Ok(())
    }
    async fn receive(&mut self) -> Result<bool, Error> {
        let r = self.server.read_frame().await?;
        if r.is_none() {
            Ok(false)
        } else {
            let m = Message::new(r.unwrap());
            let d = parse_data(MessageSource::Client, m.cid, &m.data);
            println!("rsp: {:?} {:?}", m, d);
            Ok(true)
        }
    }
    async fn get_session(uid: i32, password: &'static str) -> Result<[u8; 16], Error> {
        let mut server = Self::connect("118.89.150.23:1863").await?;
        let mut password_md = [0; 32];
        let password_md5 = format!("{:x}", md5::compute(format!("{:x}", md5::compute(password))));
        password_md.writer().write(password_md5.as_bytes())?;
        let req = LoginGetSessionReq {
            password: password_md,
            revise_tm_cid: 65,
            product_id: 10,
            zero: 0,
            verify_img_id: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            verify_code: [0, 0, 0, 0, 0, 0],
            top_left_tm_cid: [48, 0, 0, 0, 0, 0, 0, 0],
        };
        let m = Message {
            len: 0,
            cid: LoginGetSessionReq::command().cid(),
            uid,
            seq: 0,
            code: 0,
            data: Message::serialize(&req).unwrap(),
        };
        server.write(m.bytes().chunk()).await.unwrap();
        let r = server.read_frame().await?;
        let mut m = Message::new(r.unwrap());
        let rsp: Result<LoginGetSessionRsp, _> = Message::deserialize(&mut m.data);
        println!("rsp: {:?}", rsp);
        match rsp {
            Ok(rsp) => {
                Ok(rsp.session)
            }
            Err(d) => {
                eprintln!("login error: {}", d);
                exit(1);
            }
        }
    }
    async fn connect(addr: &'static str) -> Result<Connection, Error> {
        let stream = TcpSocket::new_v4()?.connect(addr.parse().unwrap()).await?;
        let addr = stream.local_addr()?;
        Ok(Connection::new(stream, addr))
    }
}
