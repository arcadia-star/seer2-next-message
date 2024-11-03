use super::*;

cmd_object! {
    struct LoginServerInfo {
        id: u16,
        ip: CString<16>,
        port: u16,
        user_cnt: i32,
        friend_cnt: i8,
        fresh_only: i8,
    }
}

cmd_object! {
    LoginGetVerifyCode {
        Client {}
        Server {
            need_verify: i32,
            verify_img_id: Hex<16>,
            verify_img_data: Vec<u8>,
        }
    }
    LoginGetSession {
        Client {
            password: Hex<32>,
            revise_tm_cid: i32,
            product_id: i32,
            zero: i32,
            verify_img_id: Hex<16>,
            verify_code: Hex<6>,
            top_left_tm_cid: CString<64>,
        }
        Server {
            zero: i32,
            session: Hex<16>,
            has_role: i32,
        }
    }
    LoginActiveCodeFail { Server {} }
    LoginGetServerList {
        Client {
            session: Hex<16>,
            revise_tm_cid: i32,
        }
        Server {
            total: i32,
            servers: Vec<LoginServerInfo>,
            fresh: i8,
            buddy_uids: Vec<i32>,
            black_uids: Vec<i32>,
        }
    }
    LoginGetServerRanged {
        Client {
            start: i16,
            end: i16,
        }
        Server {
            servers: Vec<LoginServerInfo>,
            fresh: i8,
            buddy_uids: Vec<i32>,
            black_uids: Vec<i32>,
        }
    }
    LoginCreateRole {
        Client {
            session: Hex<16>,
            revise_tm_cid: u32,
            nick: CString<16>,
            color: u32,
            sex: u8,
            top_left_tm_cid: Hex<64>,
        }
        Server {}
    }
    LoginCheckCreateRole {Server {}}
    LoginCheckDbRole {
        Client {
            session: Hex<16>,
        }
        Server {
            role: i32,
        }
    }
}
