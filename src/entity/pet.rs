use super::*;

cmd_object! {
    struct PetUserInfo {
        pid: u32,
        monster: u32,
        sex: u8,
        level: u8,
        character: u32,
        potential: u32,
        flag: u32,
        rider_chip: u32,
        rider_chip_time: u32,
        evolve_level: u32,
    }
    struct PetSimpleInfo {
        pid: u32,
        monster: u32,
        level: u16,
        state: u32,
    }
    struct PetBaseInfo {
        pid: u32,
        sex: u8,
        level: u16,
        hp: u32,
        max_hp: u32,
        atk: u16,
        sp_atk: u16,
        def: u16,
        sp_def: u16,
        spd: u16,
        exp_to_level_up: u32,
        character: u16,
        monster: u32,
    }
    struct PetStarSoulInfo {
        r#type: u32,
        level: u32,
    }
    struct PetInfo {
        base_info: PetBaseInfo,
        flag: u32,
        point_unused: u16,
        point_hp: u16,
        point_atk: u16,
        point_sp_atk: u16,
        points_def: u16,
        point_sp_def: u16,
        point_spd: u16,
        skill_ids: Vec<u32>,
        candidate_skill_ids: Vec<u32>,
        potential: u32,
        battle_level: u32,
        potential_atk: u32,
        potential_def: u32,
        potential_sp_atk: u32,
        potential_sp_def: u32,
        potential_spd: u32,
        potential_hp: u32,
        star_souls: Vec<PetStarSoulInfo>,
        height: u16,
        weight: u16,
        emblem_id: u32,
        training_count: u16,
        decoration_id: u32,
        ride_chip: u32,
        ride_chip_time: u32,
        evolve_level: u32,
    }
    struct PetStorageInfo {
        pid: u32,
        monster: u32,
        level: u16,
        treasure_time: u32,
        evolve_level: u32,
    }
    struct PetDetailInfo {
        flag: u32,
        height: u16,
        weight: u16,
        potential: u32,
        emblem: u32,
        decoration: u32,
        evolve_level: u32,
    }
    struct PetSpawnLobbyInfo {
        index: u32,
        monster: u32,
        level: u16,
        _a: u32,
        _b: u8,
    }
    struct PetStarLevelInfo {
        uid: u32,
        id: u32,
        buff: u32,
        time: u32,
        level: u32,
        exp: u32,
        next_exp: u32,
        max_level: u32,
        level_conf: u32,
        pos: u32,
        pid: u32,
        sell_exp: u32,
        r#type: u32,
        buff_swf: u32,
    }
    struct PetSptInfo {
        spt_id: u32,
        status: u8,
    }
    struct PetItemInfo {
        id: u32,
        count: u32,
    }
    struct PetItemUpdateInfo {
        pid: u32,
        item: Vec<PetItemInfo>,
    }
    struct PetMapRareInfo {
        index: u32,
        id: u32,
        level: u16,
        time: u32,
        r#type: u8,
    }
    struct PetFreeInfo {
        pid: u32,
        monster: u32,
        level: u16,
        free_time: u32,
        evolve_level: u32,
    }
    struct PetItemUseInfo {
        item_id: i32,
        item_cnt: i32,
    }
    struct PetUseItemInfo {
        pid: u32,
        items: Vec<PetItemUseInfo>,
    }
    struct PetAddLearningPointInfo {
        index: u8,
        changed: i16,
    }
    struct PetLearningPointInfo {
        index: u8,
        ability_value: u16,
        learning_point: u16,
    }
}

cmd_object! {
    PetGetStorage {
        Client {
            start: i32,
            end: i32,
        }
        Server {
            pets: Vec<PetStorageInfo>,
        }
    }
    PetGetSimple {
        Client {
            pid: u32,
        }
        Server {
            base_info: PetBaseInfo,
            detail_info: PetDetailInfo,
            _a: [u16; 21],
        }
    }
    PetSetFighting {
        Client {
            pid: u32,
        }
        Server {
            second_pet: i32,
            first_pet: i32,
        }
    }
    PetSetFollowing {
        Client {
            pid: u32,
            r#type: i8,
        }
        Server {
            uid: i32,
            pid: u32,
            r#type: i8,
        }
    }
    PetSetStorage {
        Client {
            pid: u32,
            r#type: i8,
        }
        Server {
            first_pet: i32,
            pid: u32,
            status: i8,
        }
    }
    PetSetFree {
        Client {
            pid: u32,
            monster: i32,
            flag: i32,
        }
        Server {

        }
    }
    PetGetStarLevel {
        Client {
            pid: u32,
        }
        Server {
            star: Vec<PetStarLevelInfo>,
        }
    }
    PetGetSptInfo {
        Client {}
        Server {
            spt: Vec<PetSptInfo>,
        }
    }
    PetDicRewardStatus {
        Client {}
        Server {
            data: [u64;25],
        }
    }
    PetStartOnHook {
        Client {
            start: i32,
        }
        Server {
            res: i32,
        }
    }
    PetGetFreeStorage {
        Client {}
        Server {
            pets: Vec<PetFreeInfo>,
        }
    }
    PetNumberIdCheckHave {
        Client {
            number_ids: Vec<u32>,
        }
        Server {
            data: Vec<u32>,
        }
    }
    PetGetItemList {
        Client {}
        Server {
            items: Vec<PetUseItemInfo>,
        }
    }
    PetReplaceSkill {
        Client {
            pid: u32,
            skill_ids: Vec<u32>,
        }
        Server {
            pid: u32,
            skill_ids: Vec<u32>,
            candidate_skill_ids: Vec<u32>,
        }
    }
    PetAddLearningPoint {
        Client {
            pid: u32,
            add: Vec<PetAddLearningPointInfo>
        }
        Server {
            pid: u32,
            points: Vec<PetLearningPointInfo>,
            unused_point: u16,
        }
    }
    PetCleanPotential {
        Client {
            pid: u32,
            _a: u32,
        }
        Server {
            change: [u32; 6],
            base: [u32; 6],
        }
    }
}
