use demo::{ClientOptions, Secrets};
use message::entity::{UserActivityCountReq, UserActivityCountRsp};
use once_cell::sync::Lazy;
use std::io::Error;

static CONFIG: Lazy<Secrets> = Lazy::new(|| Secrets::from_env());

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let mut client = ClientOptions::new(CONFIG.uid, &CONFIG.password).build();
    let (mut conn, rsp) = client.login().await?;
    println!("{:?}", rsp);
    let rsp: UserActivityCountRsp = conn
        .send_then_wait(UserActivityCountReq {
            activity_ids: (6000..6100).collect(),
        })
        .await?;
    println!("{:?}", rsp);
    Ok(())
}
