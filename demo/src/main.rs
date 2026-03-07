use demo::log_message;
use futures::{SinkExt, StreamExt};
use message::entity::LoginGetServerListRsp;
use message::message::{Body, Command, Message};
use message::net::{FlashPolicyCodec, MessageCodec};
use message::utils::CString;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_util::codec::Framed;
use tracing::{error, info, warn};

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

static SERVER: Lazy<Mutex<HashMap<u32, String>>> = Lazy::new(|| Default::default());
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", CONFIG.port)).await?;
    info!("listening:{}", CONFIG.port);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!(?addr, "connection accept");
        tokio::spawn(async move {
            if let Err(e) = process_connection(stream, addr).await {
                error!(?addr, "connection error, {}", e);
            } else {
                info!(?addr, "connection close");
            }
        });
    }
}

async fn process_connection(steam: TcpStream, addr: SocketAddr) -> Result<(), Error> {
    let mut framed = Framed::new(steam, FlashPolicyCodec);
    if let Some(res) = framed.next().await {
        match res {
            Ok(has) => {
                if has {
                    info!(?addr, "flash policy: hit");
                } else {
                    info!(?addr, "flash policy: skip");
                }
                framed.send(has).await?;
            }
            Err(e) => {
                warn!(?addr, "flash policy: error");
                return Err(e);
            }
        }
    }
    let mut framed = framed.map_codec(|_| MessageCodec);
    let mut framed1;
    match framed.next().await {
        None => {
            return Err(Error::new(ErrorKind::Other, "first frame is empty"));
        }
        Some(Ok(msg)) => {
            log_message(true, &addr, &msg);
            let proxy = if msg.cid == Command::UserLoginOnline.cid() {
                match SERVER.lock().await.get(&msg.uid) {
                    Some(addr) => addr.clone(),
                    None => {
                        return Err(Error::new(ErrorKind::Other, "server is empty"));
                    }
                }
            } else {
                CONFIG.login.to_string()
            };
            framed1 = Framed::new(TcpStream::connect(&proxy).await?, MessageCodec);
            framed1.send(msg).await?;
        }
        Some(Err(e)) => return Err(e),
    }

    loop {
        tokio::select! {
            client = framed.next() => {
                if let Some(res) = client {
                    match res {
                        Ok(msg) => {
                            log_message(true, &addr, & msg);
                            framed1.send(msg).await?;
                        }
                        Err(e) => {
                            warn!(?addr, "client error");
                            return Err(e);
                        }
                    }
                } else {
                    info!(?addr, "client empty");
                    break;
                }
            }
            server = framed1.next() => {
                if let Some(res) = server {
                    match res {
                        Ok(mut msg) => {
                            log_message(false, &addr, &msg);
                            may_record_proxy(&mut msg).await;
                            framed.send(msg).await?;
                        }
                        Err(e) => {
                            warn!(?addr, "server error");
                            return Err(e);
                        }
                    }
                } else {
                    info!(?addr, "server empty");
                    break;
                }
            }
        }
    }
    Ok(())
}

async fn may_record_proxy(msg: &mut Message) {
    if msg.cid != Command::LoginGetServerList.cid() {
        return;
    }
    if let Ok(mut rsp) = LoginGetServerListRsp::from_bytes(&mut msg.body.clone()) {
        if let Some(server) = rsp.servers.get(0) {
            let addr = format!("{}:{}", server.ip, server.port);
            info!("prefer server addr {}", addr);
            let mut map = SERVER.lock().await;
            map.insert(msg.uid, addr);

            rsp.servers[0].id = 1;
            rsp.servers[0].ip = CString::new("127.0.0.1");
            rsp.servers[0].port = CONFIG.port;
            if let Ok(bytes) = rsp.to_bytes() {
                msg.body = bytes;
            }
        }
    }
}
