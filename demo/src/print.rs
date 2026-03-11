use message::entity::{client_parser, server_parser};
use message::message::{error_map, Message, Parser};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::Deref;

static CLIENT_PARSER: Lazy<HashMap<u16, Parser>> =
    Lazy::new(|| client_parser().into_iter().collect());
static SERVER_PARSER: Lazy<HashMap<u16, Parser>> =
    Lazy::new(|| server_parser().into_iter().collect());
static ERROR_MAP: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| error_map());

pub trait PrintMessage {
    fn error_msg(&self) -> String;
    fn data_msg(&self, client: bool) -> String;
}

impl PrintMessage for Message {
    fn error_msg(&self) -> String {
        if self.code == 0 {
            "success".to_string()
        } else {
            match ERROR_MAP.get(&self.code) {
                None => {
                    format!("unknown-[{}]", self.code)
                }
                Some(msg) => msg.to_string(),
            }
        }
    }
    fn data_msg(&self, client: bool) -> String {
        let msg = self;
        match (if client {
            CLIENT_PARSER.deref()
        } else {
            SERVER_PARSER.deref()
        })
        .get(&self.cid)
        {
            Some(parser) => {
                let mut bytes = self.body.clone();
                match parser(&mut bytes) {
                    Ok(body) => {
                        format!(
                            "len:{} cid:{} uid:{} seq:{} code:{} body:{:?}",
                            msg.len, msg.cid, msg.uid, msg.seq, msg.code, body
                        )
                    }
                    Err(err) => {
                        format!(
                            "len:{} cid:{} uid:{} seq:{} code:{} hex:{} error:{err}",
                            msg.len,
                            msg.cid,
                            msg.uid,
                            msg.seq,
                            msg.code,
                            hex::encode(&msg.body)
                        )
                    }
                }
            }
            None => {
                format!(
                    "len:{} cid:{} uid:{} seq:{} code:{} hex:{}",
                    msg.len,
                    msg.cid,
                    msg.uid,
                    msg.seq,
                    msg.code,
                    hex::encode(&msg.body)
                )
            }
        }
    }
}
