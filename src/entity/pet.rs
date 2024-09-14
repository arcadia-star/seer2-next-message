use super::*;

cmd_object! {
    struct PetUserInfo {
        pid: i32,
        monster: i32,
        sex: i8,
        level: u8,
        character: i32,
        potential: i32,
        flag: i32,
        rider_chip: i32,
        rider_chip_time:i32,
        evolve_level: i32,
    }
    struct PetSimpleInfo {
        pid: u32,
        monster: u32,
        level: u16,
        state: u32,
    }
    struct PetBaseInfo {
        pid: i32,
        sex: i8,
        level: i16,
        hp: i32,
        mhp: u32,
        atk: u16,
        spk: u16,
        def: u16,
        spf: u16,
        spd: u16,
        exp_to_level_up: i32,
        character: i16,
        monster: i32,
    }
    struct PetStarSoulInfo {
        r#type: u32,
        level: u32,
    }
    struct PetInfo {
        base_info: PetBaseInfo,
        flag: i32,
        points_unused: u16,
        points_mhp: u16,
        points_atk: u16,
        points_spk: u16,
        points_def: u16,
        points_spf: u16,
        points_spd: u16,
        skill_ids: Vec<i32>,
        candidate_skill_ids: Vec<i32>,
        potential_origin: i32,
        battle_level: u32,
        potential_atk: i32,
        potential_def: i32,
        potential_spk: i32,
        potential_spf: i32,
        potential_spd: i32,
        potential_mhp: i32,
        star_souls: Vec<PetStarSoulInfo>,
        height: i16,
        weight: i16,
        emblem_id: i32,
        training_count: u16,
        decoration_id: i32,
        rider_chip: i32,
        rider_chip_time: u32,
        evolve_level: i32,
    }
    struct PetStorageInfo {
        pid: i32,
        monster: i32,
        level: i16,
        treasure_time: i32,
        evolve_level: i32,
    }
    struct PetDetailInfo {
        flag: i32,
        height: i16,
        weight: i16,
        potential: i32,
        emblem: i32,
        decoration: i32,
        evolve_level: i32,
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
        pid: i32,
        monster: i32,
        level: i16,
        free_time: i32,
        evolve_level: i32,
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
    struct PetTrainingInfo {
        monster: i32,
        pid: i32,
        level: i32,
        character: i32,
        battle_potential: i32,
        start_training_time:i32,
        fight_exp: i32,
        mhp: i32,
        atk: u16,
        spk: u16,
        def: u16,
        spf: u16,
        spd: u16,
        skills: Vec<i32>,
        flag: i32,
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
            pid: i32,
        }
        Server {
            base_info: PetBaseInfo,
            detail_info: PetDetailInfo,
            _a: [u16; 21],
        }
    }
    PetSetFighting {
        Client {
            pid: i32,
        }
        Server {
            second_pet: i32,
            first_pet: i32,
        }
    }
    PetSetFollowing {
        Client {
            pid: i32,
            flag: i8,
        }
        Server {
            uid: i32,
            pid: i32,
            flag: i8,
            data: Vec<PetUserInfo>,
        }
    }
    PetPutStorage {
        Client {
            pid: i32,
            put_flag: i8,
        }
        Server {
            first_pid: i32,
            pid: i32,
            status: u8,
            pets: Vec<PetInfo>,
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
    PetRebornPotential {
        Client {
            pid: i32,
            flag: i32,
        }
        Server {
            change: [i32; 6],
            base: [i32; 6],
        }
    }
    PetChangeCharacter {
        Client {
            pid: i32,
            item: i32,
        }
        Server {
            pet: PetBaseInfo,
            item: i32,
        }
    }
    PetBuySkill {
        Client {
            pid: i32,
            skills: Vec<i32>,
        }
        Server {
            skills: Vec<i32>,
        }
    }
}
