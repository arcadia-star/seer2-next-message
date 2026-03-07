use demo::{Client, Secrets};
use message::entity::{UserActivityCountReq, UserActivityCountRsp};
use once_cell::sync::Lazy;
use std::io::Error;

static CONFIG: Lazy<Secrets> = Lazy::new(|| Secrets::from_env());

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let (mut client, _) = Client::login(CONFIG.uid, &CONFIG.password).await?;
    let rsp: UserActivityCountRsp = client
        .send(UserActivityCountReq {
            activity_ids: (6000..6100).collect(),
        })
        .await?;
    println!("{:?}", rsp);
    Ok(())
}
