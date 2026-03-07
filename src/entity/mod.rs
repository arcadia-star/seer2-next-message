use crate::utils::CString;
use crate::utils::Hex;
use crate::utils::UTFString;
macro_rules! cmd_object_mods {
    ($($m:ident),+) => {
        $(
            mod $m;
            pub use $m::*;
        )+
        pub fn client_parser()-> Vec<(u16, crate::message::Parser)>{
            vec![
                $( $m::client_parser(), )+
            ].into_iter().flat_map(|e|e.into_iter()).collect()
        }
        pub fn server_parser()-> Vec<(u16, crate::message::Parser)>{
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

            pub(super) fn client_parser()-> Vec<(u16, crate::message::Parser)> {
               vec![
                   $($(cmd_object!{@parser $cmd, [<$cmd Req>], [$($client_field)*] },)?)*
               ]
            }
            pub(super) fn server_parser()-> Vec<(u16, crate::message::Parser)> {
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
            crate::message::Command::$cmd.cid(),
            |data: &mut bytes::Bytes| {
                use crate::message::Body;
                Ok(Box::new($ident::from_bytes(data)?))
            }
        )
    };
    (@message $ident:ident $cmd:ident) => {
        impl $ident {

        }
        impl crate::message::Body for $ident {
            fn command() -> crate::message::Command {
                crate::message::Command::$cmd
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn from_bytes(bytes: &mut bytes::Bytes) -> Result<Self, crate::message::SerdeError> {
                crate::message::from_bytes(bytes)
            }
            fn to_bytes(&self) -> Result<bytes::Bytes, crate::message::SerdeError> {
                crate::message::to_bytes(self)
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
