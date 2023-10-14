macro_rules! cmd_object_mods {
    ($($m:ident),+) => {
        $(
            mod $m;
            pub use $m::*;
        )+
        pub(crate) fn get_all_parsers()-> Vec<Box<dyn crate::message::MessageParserTrait>>{
            vec![
                $( $m::get_all_parser(), )+
            ].into_iter().flat_map(|e|e.into_iter()).collect()
        }
    };
}

cmd_object_mods![fight, item, login, notify, pet, quest, team, user, vip];

macro_rules! cmd_object {
    ( $(
        struct $name:ident {
            $( $( #[$field_attr:meta] )*  $field:ident : $typ:ty ),* $(,)?
        }
    )+ ) => {
        $(#[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct $name {
            $( $( #[$field_attr] )* pub $field : $typ, )*
        })+
    };
    ($($cmd:ident{
        $(Client {$( $( #[$field_attr:meta] )*  $field:ident : $typ:ty ),* $(,)?})?
        Server {$( $( #[$field_attr1:meta] )*  $field1:ident : $typ1:ty ),* $(,)?}
    })*)=>{
        paste::paste!{
            $(cmd_object!{@msg
                $(@($cmd, Client)
                struct [<$cmd Req>] {
                     $( $( #[$field_attr] )* $field : $typ, )*
                })?
                @($cmd, Server)
                struct [<$cmd Rsp>] {
                     $( $( #[$field_attr1] )* $field1 : $typ1, )*
                }
            })*
            pub(super) fn get_all_parser()-> Vec<Box<dyn crate::message::MessageParserTrait>>{
               cmd_object!{@vec
               $(
                   $([<$cmd Req Parser>]{}, @ {$($field)*} )?
                   [<$cmd Rsp Parser>]{},
               )*
               }
            }
        }
    };
    (@msg $(@($cmd:ident, $src:ident) struct $name:ident {
        $( $( #[$field_attr:meta] )*  $field:ident : $typ:ty ),* $(,)?
    })*)=>{$(paste::paste!{
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct $name {
            $( $( #[$field_attr] )* pub $field : $typ, )*
        }
        impl $name {
            pub fn command() -> crate::message::MessageCommand {
                crate::message::MessageCommand::$cmd
            }
            pub fn source() -> crate::message::MessageSource {
                crate::message::MessageSource::$src
            }
        }
        impl crate::message::MessageTrait for $name {
            fn command(&self) -> crate::message::MessageCommand {
                crate::message::MessageCommand::$cmd
            }
            fn source(&self) -> crate::message::MessageSource {
                crate::message::MessageSource::$src
            }
        }
        struct [<$name Parser>] {}
        impl crate::message::MessageParserTrait for [<$name Parser>] {
            fn command(&self) -> crate::message::MessageCommand {
                crate::message::MessageCommand::$cmd
            }
            fn source(&self) -> crate::message::MessageSource {
                crate::message::MessageSource::$src
            }
            fn parse(&self, msg: &crate::message::Message) -> Result<Box<dyn crate::message::MessageTrait>, crate::message::SerdeError> {
                let d: $name = crate::message::Message::deserialize(&mut msg.data.clone())?;
                Ok(Box::new(d))
            }
        }
    })*};
    (@vec $($exp:expr, $(@{ $($_field:ident)* })?)*)=>{
        vec![
            $(Box::new($exp),)*
        ]
    };
}

use cmd_object;
