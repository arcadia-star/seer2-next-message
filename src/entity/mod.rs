use crate::message::MessageParser;
use crate::utils::CString;
macro_rules! cmd_object_mods {
    ($($m:ident),+) => {
        $(
            mod $m;
            pub use $m::*;
        )+
        pub(crate) fn client_parser()-> Vec<(u16, MessageParser)>{
            vec![
                $( $m::client_parser(), )+
            ].into_iter().flat_map(|e|e.into_iter()).collect()
        }
        pub(crate) fn server_parser()-> Vec<(u16, MessageParser)>{
            vec![
                $( $m::server_parser(), )+
            ].into_iter().flat_map(|e|e.into_iter()).collect()
        }
    };
}

cmd_object_mods![fight, item, login, notify, pet, quest, team, user, vip];

macro_rules! cmd_object {
    ( $(
        struct $name:ident {
            $( $field:ident : $typ:ty ),* $(,)?
        }
    )+ ) => {
        $(
        cmd_object!{@entity struct $name {$( $field : $typ ),*}}
        )+
    };
    ($($cmd:ident{
        $(Client {$( $client_field:ident : $client_typ:ty ),* $(,)? })?
        $(Server {$( $server_field:ident : $server_typ:ty ),* $(,)? })?
    })*)=>{
        paste::paste!{
            $($(
            cmd_object!{@entity struct [<$cmd Req>] { $( $client_field : $client_typ ),* }}
            cmd_object!{@message [<$cmd Req>] $cmd}
            )?)*
            $($(
            cmd_object!{@entity struct [<$cmd Rsp>] { $( $server_field : $server_typ ),* }}
            cmd_object!{@message [<$cmd Rsp>] $cmd}
            )?)*

            pub(super) fn client_parser()-> Vec<(u16, MessageParser)> {
               vec![
                   $($(cmd_object!{@parser $cmd, [<$cmd Req>], [$($client_field)*] },)?)*
               ]
            }
            pub(super) fn server_parser()-> Vec<(u16, MessageParser)> {
               vec![
                   $($(cmd_object!{@parser $cmd, [<$cmd Rsp>], [$($server_field)*] },)?)*
               ]
            }
        }
    };
    (@entity struct $ident:ident { $($field:ident: $typ:ty ),*}) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
        pub struct $ident {
            $(pub $field: $typ ),*
        }
    };
    (@parser $cmd:ident, $ident:ident, $ignore:tt) => {
        (
            crate::message::MessageCommand::$cmd,
            |data: &mut bytes::Bytes|{Ok(Box::new(<$ident as crate::message::MessageData>::from_bytes(data)?))}
        )
    };
    (@message $ident:ident $cmd:ident) => {
        impl crate::message::MessageData for $ident {
            fn command() -> u16 {
                crate::message::MessageCommand::$cmd
            }
            fn from_bytes(bytes: &mut bytes::Bytes) -> Result<Self, crate::error::Error> {
                let bak = bytes.clone();
                crate::message::serde_bytes::from_bytes(bytes)
                .map_err(|err|crate::error::Error::ParseError(err.0,bak))
            }
            fn to_bytes(&self) -> Result<bytes::Bytes, crate::error::Error> {
                crate::message::serde_bytes::to_bytes(self)
                .map_err(|err|crate::error::Error::SerdeError(err.0))
            }
            fn to_json(&self) -> Result<String, crate::error::Error> {
                serde_json::to_string(self)
                .map_err(|err|crate::error::Error::SerdeError(err.to_string()))
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

use cmd_object;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", client_parser().len());
        println!("{}", server_parser().len());
    }
}
