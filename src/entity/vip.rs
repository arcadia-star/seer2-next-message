use super::*;

cmd_object! {
    struct VipSimpleInfo {
        vip: i8,
        level: i8,
    }
    struct VipBaseInfo {
        pay: i32,
        simple: VipSimpleInfo,
        vip_point: i32,
        left_day: i32,
        equip_ball_ids: Vec<i32>,
        energy_ball: i32,
        current_energy: i32,
        total_energy: i32,
        once_vip_flag: i8,
        lucky_left_day: i8,
        lucky_msg_type: i8,
        lucky_flag: i8,
    }
}

cmd_object! {}
