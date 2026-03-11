use demo::{ClientOptions, PrintMessage, Secrets};
use message::entity::DoorFightMatchReq;
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::time::timeout;

static CONFIG: Lazy<Secrets> = Lazy::new(|| Secrets::from_env());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientOptions::new(CONFIG.uid, &CONFIG.password)
        // .proxy("127.0.0.1:7890")
        .build();
    let (mut conn, _rsp) = client.login().await?;

    for idx in 231..230 {
        for ty in 28..29 {
            match timeout(Duration::from_secs(2), async {
                conn.send_then_wait_raw(DoorFightMatchReq {
                    gate_type: ty,
                    gate: idx,
                })
                .await
                .unwrap()
            })
            .await
            {
                Ok(res) => {
                    if res.code > 0 {
                        println!("error: {} type:{ty} gate:{idx}", res.error_msg());
                    } else {
                        println!("success type:{ty} gate:{idx}");
                    }
                }
                Err(_) => {
                    println!("success match, type:{} gate:{}", ty, idx);
                    client.login().await.ok();
                    (conn, _) = client.login().await?;
                    continue;
                }
            }
        }
    }

    let rsp = conn
        .send_then_wait_raw(DoorFightMatchReq {
            gate_type: 28,
            gate: 220,
        })
        .await;
    println!("{}", rsp?.data_msg(false));
    conn.wait(1000).await?;
    Ok(())
}
