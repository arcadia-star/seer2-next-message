use super::*;

cmd_object! {
    struct UserNooInfo {
        noo_has: i8,
        name: CString<16>,
        flag: i32,
        color: i32,
    }
    struct UserNooTimeInfo {
        r#type: i8,
        start_time: u32,
        end_time: u32,
    }
}

cmd_object! {
    struct UserBaseInfo {
        uid: i32,
        nick: CString<16>,
        color: i32,
        loc_x: i32,
        loc_y: i32,
        medal: i32,
        create_time: i32,
        equips: Vec<i32>,
        noo_equips: Vec<i32>,
    }
    struct UserLoginInfo {
        user: UserBaseInfo,
        sex: i8,
        coins: i32,
        map: i32,
        scene: i32,
        score: i32,
        max_pet_level: u8,
        login_time: u32,
        completed_quests: Vec<i32>,
        last_completed_quests: Vec<i32>,
        progressing_quests: Vec<i32>,

        fight_pets: Vec<PetInfo>,
        storage_pets: Vec<PetInfo>,
        other_pets: Vec<PetSimpleInfo>,

        max_available_time: i32,
        available_time: i32,
        double_exp_time: i32,

        show_login_award_panel: i32,
        caller_state_flag: i32,
        caller_user_id: i32,
        team_base_info: TeamBaseInfo,
        team_ext_info: TeamExtInfo,
        honor_num: i32,

        vip_info: VipBaseInfo,

        noo_info: UserNooInfo,
        noo_time_infos: Vec<UserNooTimeInfo>,

        expired_equips: Vec<i32>,

        online_days: i32,
        gift_index: i32,
        summer_registration_days: i32,
        birthday_info: i32,

        ip: CString<20>,
        year_vip: i32,
    }
    struct UserEnterDetailInfo {
        user_info: UserBaseInfo,
        pets: Vec<PetUserInfo>,
        troop: i32,
        vip: VipSimpleInfo,
        activity: Vec<i32>,
        score: i32,
    }
    struct UserOnlineInfo {
        user_info: UserEnterDetailInfo,
        map_user_status: i8,
        noo_info: UserNooInfo,
        morph_id: i32,
        birthday: i32,
        year_vip: i32,
    }
    struct UserSimpleInfo {
        uid: i32,
        sex: u8,
        nick: CString<16>,
        color: i32,
        score: i32,
        equips: Vec<i32>,
        vip: VipSimpleInfo,
        plant_lv: i32,
    }
    struct UserDetailInfo {
        user_info: UserSimpleInfo,
        create_time: i32,
        medal_id: i32,
        pet_count: i32,
        pet_level: i8,
        spt_count: i32,
        medal_count: i32,
        signature: Vec<u8>,
        team: TeamBaseInfo,
    }
    struct UserServerInfo {
        uid: i32,
        server: i16,
    }
}

cmd_object! {
    struct UserDayLimitInfo {
        limit_id: i32,
        count: i32,
    }
    struct UserRankInfo {
        uid: i32,
        _a: u32,
        rank: i32,
        _b: u32,
        time: i32,
        score: i32,
        nick: String,
    }
    struct UserTimeRange {
        start: u32,
        end: u32,
    }
    struct UserDayExpInfo {
        bit_time: u32,
        exping_time: u32,
        exp_rate: u32,
        ev_rate: u32,
        time: Vec<UserTimeRange>,
    }
    struct UserMailInfo {
        mail_id: i32,
        raw_send_time: i32,
        has_read: i32,
        r#type: i32,
        attachment_symble: i32,
        sender_id: i32,
        sender_name: String,
        mail_title: String,
    }
    struct UserHomeHonorSptInfo {
        id: u32,
        status: u8,
    }
    struct UserHomeHonorGateInfo {
        id: u8,
        pve_normal: u8,
        pve_fifty: u8,
        pve_last_normal: u8,
        pve_last_fifty: u8,
    }
}

cmd_object! {
    UserLoginDeputize {
        Client {
            ip: [u8;4],
            port: u16,
        }
        Server {}
    }
    UserLoginOnline {
        Client {
            from_game: i32,
            session: Hex<16>,
            top_left_tm_cid: CString<64>,
        }
        Server {
            login: UserLoginInfo,
        }
    }
    UserEnterMap {
        Client {
            map: i32,
            scene: i32,
            x: i32,
            y: i32,
            behavior: i32,
        }
        Server {
            detail: UserEnterDetailInfo,
            map_user_status: i8,
            noo_info: UserNooInfo,
            morph_id: i32,
            birthday: i32,
            year_vip: i32,
        }
    }
    UserLeaveMap {
        Client {}
        Server {
            uid: i32,
            fight: i8,
        }
    }
    UserOnlineList {
        Client {}
        Server {
            users: Vec<UserOnlineInfo>,
        }
    }
    UserChangeMedal {
        Client {}
        Server {
            uid:i32,
            medal_id:i32,
        }
    }
    UserOnlineDetail {
        Client {
            uid: i32,
        }
        Server {
            user_info: UserDetailInfo,
            morph_id: i32,
            morph_end_time: i32,
        }
    }
    UserBuddyOnline {
        Client {
            uids: Vec<i32>,
        }
        Server {
            users: Vec<UserServerInfo>,
        }
    }
    UserBuddyAdd {
        Client {
            uid: i32,
            black: i8,
        }
        Server {
            user: UserSimpleInfo,
            black: i8,
            server: i8,
        }
    }
    UserBuddyRemove {
        Client {
            uid: i32,
            black: i8,
        }
        Server {
            uid: i32,
            nick: CString<16>,
            black: i8,
        }
    }
    UserOnlineSimple {
        Client {
            uid: i32,
        }
        Server {
            user_info: UserSimpleInfo,
        }
    }
    UserDayLimitList {
        Client {
            limit_ids: Vec<i32>,
        }
        Server {
            data: Vec<UserDayLimitInfo>,
        }
    }
    UserBatterySwitch {
        Client {
            open: u32,
        }
        Server {
            available_time: u32,
        }
    }
    UserGetRankList {
        Client {
            rank_id: u32,
            min: u32,
            max: u32,
        }
        Server {
            max_id: u32,
            mid_id: u32,
            min_id: u32,
            rank: Vec<UserRankInfo>,
        }
    }
    UserGetActorRank {
        Client {
            rank_type: i32,
            uid: i32,
        }
        Server {
            _a: u32,
            _b: u32,
            _c: u32,
            rank: UserRankInfo,
        }
    }
    UserAllServerData {
        Client {
            data: Vec<u32>,
        }
        Server {
            data: Vec<u32>,
        }
    }
    UserDayLimitSingle {
        Client {
            limit_id: i32,
        }
        Server {
            data: UserDayLimitInfo,
        }
    }
    UserDayExpTime {
        Client {}
        Server {
            day0: UserDayExpInfo,
            day1: UserDayExpInfo,
            day2: UserDayExpInfo,
            day3: UserDayExpInfo,
            day4: UserDayExpInfo,
            day5: UserDayExpInfo,
            day6: UserDayExpInfo,
        }
    }
    UserPositionSync {
        Client {
            loc_x: i32,
            loc_y: i32,
            time: u32,
        }
        Server {
            uid: i32,
            loc_x: i32,
            loc_y: i32,
        }
    }
    UserSession {
        Client {
            product_id: u32,
        }
        Server {
            session: Hex<16>,
        }
    }
    UserActivityCount {
        Client {
            activity_ids: Vec<i32>,
        }
        Server {
            data: Vec<i32>,
        }
    }
    UserBufferRead {
        Client {
            id: i32,
        }
        Server {
            id: i32,
            data: Hex<50>,
        }
    }
    UserBufferWrite {
        Client {
            id: i32,
            data: Hex<50>,
        }
        Server {}
    }
    UserMailList {
        Client {
            uid: u32,
        }
        Server {
            mails: Vec<UserMailInfo>,
        }
    }
    UserChat {
        Client {
            uid: u32,
            msg: String,
        }
        Server {
            uid: u32,
            nick: CString<16>,
            receiver: u32,
            pipe: u8,
            msg: String,
        }
    }
    UserHomeHonor {
        Client {
            uid: u32,
        }
        Server {
            _a: Vec<u32>,
            spt: Vec<UserHomeHonorSptInfo>,
            gate: Vec<UserHomeHonorGateInfo>,
        }
    }
    UserHomeScene {
        Client {}
        Server {
            left_fight_cnt: u8,
            training_pets: Vec<PetTrainingInfo>,
            garbage_cnt: u8,
            birth_pets: Vec<PetInfo>,
            sule_award_cnt: i32,
            semiya_cnt: i32,
        }
    }
    UserShoot {
        Client {
            event: i32,
            shot: i32,
            x: i32,
            y: i32,
        }
        Server {
            uid: i32,
            event:i32,
            shot: i32,
            x: i32,
            y: i32,
        }
    }
    UserChangeEquip {
        Client {
            equips: Vec<i32>,
            noo_equips: Vec<i32>,
        }
        Server {
            uid: i32,
            equips: Vec<i32>,
            noo_equips: Vec<i32>,
        }
    }
    UserChangeNick {
        Client {}
        Server {}
    }
    UserChangeSignature {
        Client {}
        Server {}
    }
    UserDigMine {
        Client {
            mine: i32,
        }
        Server {
            reward: ItemReward,
        }
    }
}
