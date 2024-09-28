use std::collections::HashMap;
use std::fs;
use std::io::Error;
use std::sync::Arc;

use bytes::Buf;
use log::{error, info};
use message::entity::LoginGetServerListRsp;
use message::message::{Message, MessageCommand};
use message::net::Connection;
use message::utils::CString;
use once_cell::sync::Lazy;
use tokio::net::{TcpListener, TcpSocket};
use tokio::select;
use tokio::sync::Mutex;

struct Config {
    port: u16,
    login: String,
}
static CONFIG: Lazy<Config> = Lazy::new(|| {
    let map: HashMap<String, String> = dotenv::vars().collect();
    Config {
        port: map.get("LOCAL_PORT").unwrap().parse().unwrap(),
        login: map.get("LOGIN_SERVER").unwrap().clone(),
    }
});

#[derive(PartialEq)]
enum MessageSource {
    Client,
    Server,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger().expect("init log error");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", CONFIG.port)).await?;
    info!("listening:{:?}", CONFIG.port);
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

async fn handle_each(mut client: Connection, map: Arc<Mutex<HashMap<u32, (String, u16)>>>) -> Result<(), Error> {
    let mut client = {
        //flash policy
        if client.discard_flash_policy().await? {
            info!("flash policy: hit");
        } else {
            info!("flash policy: skip");
        }
        client
    };
    let bytes = client.read_frame().await?;
    if bytes.is_none() {
        error!("none after flash policy");
        return Ok(());
    }
    let bytes = bytes.unwrap();

    let msg = Message::parse_client(&mut bytes.clone());
    parse_msg(MessageSource::Client, &msg);

    let server_addr = if msg.cid == MessageCommand::UserLoginOnline {
        let mut lock = map.lock().await;
        let addr = lock.remove(&msg.uid);
        match addr {
            None => unreachable!(),
            Some((ip, port)) => {
                format!("{}:{}", ip, port)
            }
        }
    } else {
        CONFIG.login.to_string()
    };

    let mut server = {
        let server_socket = TcpSocket::new_v4()?;
        let server_stream = server_socket.connect(server_addr.parse().unwrap()).await?;
        info!("connected:{}", server_addr);
        let addr = server_stream.local_addr()?;
        let mut server = Connection::new(server_stream, addr);
        server.write(bytes.chunk()).await?;
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
                parse_msg(MessageSource::Client, &Message::parse_client(&mut b.clone()));
                server.write(b.chunk()).await?;
            }
            b = server.read_frame() =>{
                let b = b.unwrap();
                if b.is_none() {
                    error!("server->client:closed");
                    break;
                }
                let mut b = b.unwrap();
                parse_msg(MessageSource::Server, &Message::parse_server(&mut b.clone()));
                let t = b.get(4..=5);
                if let Some(cmd) = t {
                    if cmd == [105, 0] {
                        let m = Message::parse_server(&mut b);
                        let d: &LoginGetServerListRsp = m.body.downcast_ref().unwrap();
                        {
                            let mut lock = map.lock().await;
                            lock.insert(m.uid, (d.servers[1].ip.to_string(), d.servers[1].port));
                        }
                        let mut d = d.clone();
                        d.servers[1].id = 1;
                        d.servers[1].ip = CString::new("127.0.0.1");
                        d.servers[1].port = CONFIG.port;
                        b = Message::new(m.uid, d).to_server_bytes();
                    }
                }
                client.write(b.chunk()).await?;
            }
        }
    }
    Ok::<(), Error>(())
}

const RAW: &str = "RAW";

fn parse_msg(src: MessageSource, m: &Message) {
    let prefix = match src {
        MessageSource::Client => "C:",
        MessageSource::Server => "S:",
    };
    let len = m.len;
    let cid = m.cid;
    let uid = m.uid;
    let seq = m.seq;
    let code = m.code;
    match m.body.deref() {
        Ok(data) => {
            info!(target: RAW,"{prefix} success [{len},{cid},{uid},{seq},{code}] {} {:?}", m.body.to_hex() ,data);
        }
        Err(_) => {
            error!(target: RAW,"{prefix} unknown [{len},{cid},{uid},{seq},{code}] {}", m.body.to_hex());
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