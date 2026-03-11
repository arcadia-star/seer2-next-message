use demo::ClientConnection;
use message::entity::{LoginGetServerRangedReq, LoginGetServerRangedRsp};
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    // let addr = "118.89.150.23:1863";
    let addr = "118.89.150.43:1863";
    let mut client = ClientConnection::connect(addr, 1).await?;
    let msg: LoginGetServerRangedRsp = client
        .send_then_wait(LoginGetServerRangedReq { start: 1, end: 100 })
        .await?;
    for x in msg.servers {
        println!("{:?}", x);
    }
    Ok(())
}
