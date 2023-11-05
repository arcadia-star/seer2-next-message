use std::collections::HashMap;
use std::fs;
use std::io::{Error, Write};
use std::sync::Arc;

use bytes::{Buf, BufMut, Bytes};
use log::{error, info};
use tokio::net::{TcpListener, TcpSocket};
use tokio::select;
use tokio::sync::Mutex;

use crate::entity::LoginGetServerListRsp;
use crate::message::{Message, MessageSource, parse_data};
use crate::net::Connection;

#[allow(unused)]
mod entity;
#[allow(unused)]
mod message;
#[allow(unused)]
mod net;

const PROXY_ADDR: (&str, u16) = ("127.0.0.1", 5001);
const LOGIN_ADDR: &str = "118.89.150.23:1863";

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger().expect("init log error");

    let listener = TcpListener::bind(PROXY_ADDR).await?;
    info!("listening:{:?}", PROXY_ADDR);
    let map = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let map = map.clone();
        let (socket, addr) = listener.accept().await?;
        info!("accept:{}", addr);
        tokio::task::spawn(async move {
            handle_each(Connection::new(socket, addr), map).await.unwrap();
        });
    }
}

async fn handle_each(mut client: Connection, map: Arc<Mutex<HashMap<i32, (Vec<u8>, u16)>>>) -> Result<(), Error> {
    let mut client = {
        //flash policy
        if client.discard_flash_policy().await? {
            info!("flash policy: hit");
        } else {
            info!("flash policy: skip");
        }
        client
    };
    let (server_addr, b) = {
        let b = client.read_frame().await?;
        if b.is_none() {
            error!("none after flash policy");
            return Ok(());
        }
        let msg = Message::new(b.unwrap());
        parse_msg(MessageSource::Client, msg.bytes());
        let mut lock = map.lock().await;
        let addr = lock.remove(&msg.uid);
        (
            match addr {
                None => LOGIN_ADDR.to_string(),
                Some((a, p)) => {
                    let mut v = vec![];
                    for x in a {
                        if x == 0 {
                            break;
                        }
                        v.push(x);
                    }
                    let r = format!("{}:{}", String::from_utf8(v).unwrap(), p);
                    r
                }
            },
            msg.bytes(),
        )
    };
    let mut server = {
        let server_socket = TcpSocket::new_v4()?;
        let server_stream = server_socket.connect(server_addr.parse().unwrap()).await?;
        info!("connected:{}", server_addr);
        let t = server_stream.local_addr()?;
        let mut server = Connection::new(server_stream, t);
        server.write(b.chunk()).await?;
        server
    };
    loop {
        select! {
            b = client.read_frame() =>{
                let b = b.unwrap();
                if b.is_none() {
                    error!("client->server:closed");
                    break;
                }
                let b = b.unwrap();
                parse_msg(MessageSource::Client, b.clone());
                server.write(b.chunk()).await?;
            }
            b = server.read_frame() =>{
                let b = b.unwrap();
                if b.is_none() {
                    error!("server->client:closed");
                    break;
                }
                let mut b = b.unwrap();
                parse_msg(MessageSource::Server, b.clone());
                let t = b.get(4..=5);
                if let Some(cmd) = t {
                    if cmd == [105, 0] {
                        let mut m = Message::new(b);
                        let mut d: LoginGetServerListRsp = Message::deserialize(&mut m.data).unwrap();
                        {
                            let mut lock = map.lock().await;
                            lock.insert(m.uid, (d.servers[1].ip.to_vec(), d.servers[1].port));
                        }
                        d.servers[1].id = 1i16;
                        let mut ip = [0u8; 16];
                        ip.writer().write(PROXY_ADDR.0.as_bytes()).unwrap();
                        d.servers[1].ip = ip;
                        d.servers[1].port = PROXY_ADDR.1;
                        m.data = Message::serialize(&d).unwrap();
                        b = m.bytes();
                    }
                }
                client.write(b.chunk()).await?;
            }
        }
    }
    Ok::<(), Error>(())
}

const RAW: &str = "RAW";

fn parse_msg(src: MessageSource, bytes: Bytes) {
    let prefix = match src {
        MessageSource::Client => "C:",
        MessageSource::Server => "S:",
    };
    let m = Message::new(bytes);
    let len = m.len;
    let cid = m.cid;
    let uid = m.uid;
    let seq = m.seq;
    let code = m.code;
    if MessageSource::Server == src && code > 0 {
        info!(target: RAW,"{prefix} success [{len},{cid},{uid},{seq},{code}] {}",hex::encode(m.data.chunk()));
        return;
    }
    match parse_data(src, cid, &m.data) {
        Ok(s) => {
            match s {
                None => {
                    error!(target: RAW,"{prefix} unknown [{len},{cid},{uid},{seq},{code}] {}",hex::encode(m.data.chunk()));
                }
                Some(s) => {
                    info!(target: RAW,"{prefix} success [{len},{cid},{uid},{seq},{code}] {} {:?}",hex::encode(m.data.chunk()),s);
                }
            }
        }
        Err(e) => {
            error!("parse error {:?}",e);
            error!(target: RAW,"{prefix} error [{len},{cid},{uid},{seq},{code}] {}",hex::encode(m.data.chunk()));
        }
    }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fs::create_dir("logs").ok();
    let file_name = chrono::Local::now().format("%Y-%m-%d_%H_%M_%S_%3fS");
    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("[{} {} {}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3fS"), record.level(), record.target(), message)))
        .level(log::LevelFilter::Debug)
        .chain(
            fern::Dispatch::new()
                .filter(|e| e.target() == RAW)
                .chain(fern::log_file(format!("logs/{file_name}.msg.log"))?)
                .chain(fern::Dispatch::new().level(log::LevelFilter::Error).chain(fern::log_file(format!("logs/{file_name}.msg.err.log"))?)),
        )
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
