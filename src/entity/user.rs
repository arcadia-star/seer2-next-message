use super::*;

cmd_object! {
    struct UserNooInfo {
        noo_has: i8,
        name: [u8; 16],
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
        nick: [u8; 16],
        color: i32,
        loc_x: i32,
        loc_y: i32,
        medal: i32,
        create_time: u32,
        equip_ids: Vec<i32>,
        noo_equip_ids: Vec<i32>,
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

        expired_equip_ids: Vec<i32>,

        online_days: i32,
        gift_index: i32,
        summer_registration_days: i32,
        birthday_info: i32,

        ip: [u8; 20],
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
        sex: i8,
        nick: [u8; 16],
        color: i32,
        score: i32,
        equip_ids: Vec<i32>,
        vip_info: VipSimpleInfo,
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
        id: u32,
        count: u32,
    }
    struct UserRankInfo {
        uid: u32,
        _a: u32,
        rank: u32,
        _b: u32,
        time: u32,
        score: u32,
        nick: Vec<u8>,
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
        mail_id: u32,
        raw_send_time: u32,
        has_read: u32,
        r#type: u32,
        attachment_symble: u32,
        sender_id: u32,
        sender_name: Vec<u32>,
        mail_title: Vec<u32>,
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
            session: [u8; 16],
            top_left_tm_cid: [u64; 8],
        }
        Server {
            login: UserLoginInfo,
        }
    }
    UserEnterMap {
        Client {
            map: i32,
            scene: i32,
            loc_x: i32,
            loc_y: i32,
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
    UserLevelMap {
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
            nick: [u8; 16],
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
            ids: Vec<u32>,
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
            rank_type: u32,
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
            id: u32,
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
    UserSyncPosition {
        Client {
            loc_x: u32,
            loc_y: u32,
            time: u32,
        }
        Server {
            uid: u32,
            loc_x: u32,
            loc_y: u32,
        }
    }
    UserGetSession {
        Client {
            product_id: u32,
        }
        Server {
            session: [u8;16],
        }
    }
    UserMoneyCount {
        Client {
            data: [u8;16]
        }
        Server {
            count: u32,
        }
    }
    UserActivityCount {
        Client {
            r#type: Vec<u32>,
        }
        Server {
            data: Vec<u32>,
        }
    }
    UserGetClientBuffer {
        Client {
            r#type: u32,
        }
        Server {
            r#type: u32,
            buffer0: [u16;25],
        }
    }
    UserSetClientBuffer {
        Client {
            r#type: u32,
            buffer0: [u16;25],
        }
        Server {}
    }
    UserGetMailList {
        Client {
            uid: u32,
        }
        Server {
            mails: Vec<UserMailInfo>,
        }
    }
}
