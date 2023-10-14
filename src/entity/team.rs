use super::*;

cmd_object! {
    struct TeamBaseInfo {
        team_id: i32,
        team_name: [u8; 25],
        logo_front: i32,
        logo_back: i32,
        logo_color: i32,
    }
    struct TeamExtInfo {
        dissolve_day: i32,
        user_post: i8,
        user_total_contribute: i32,
        t1: i32,
    }
}
cmd_object! {}
