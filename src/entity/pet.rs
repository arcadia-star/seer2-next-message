use super::*;

cmd_object! {
    struct PetLearnInfo {
        unused: u16,
        hpm: u16,
        atk: u16,
        spk: u16,
        def: u16,
        spf: u16,
        spd: u16,
    }
    struct PetBornInfo {
        atk: i32,
        def: i32,
        spk: i32,
        spf: i32,
        spd: i32,
        hpm: i32,
    }
    struct PetUserInfo {
        pid: i32,
        monster: i32,
        sex: u8,
        level: u8,
        character: i32,
        potential: i32,
        flag: u32,
        ride_chip: i32,
        ride_time:i32,
        evolve_level: i32,
    }
    struct PetSimpleInfo {
        pid: i32,
        monster: i32,
        level: i16,
        state: u32,
    }
    struct PetBaseInfo {
        pid: i32,
        sex: u8,
        level: u16,
        hp: i32,
        mhp: i32,
        atk: u16,
        spk: u16,
        def: u16,
        spf: u16,
        spd: u16,
        exp_to_level_up: i32,
        character: u16,
        monster: i32,
    }
    struct PetStarSoulInfo {
        r#type: u32,
        level: u32,
    }
    struct PetInfo {
        base: PetBaseInfo,
        flag: u32,
        learn: PetLearnInfo,
        skills: Vec<i32>,
        candidate_skills: Vec<i32>,
        potential: i32,
        battle_level: u32,
        born: PetBornInfo,
        star_souls: Vec<PetStarSoulInfo>,
        height: u16,
        weight: u16,
        emblem: i32,
        training_count: u16,
        decoration: i32,
        ride_chip: i32,
        ride_time: i32,
        evolve_level: i32,
    }
    struct PetStorageInfo {
        pid: i32,
        monster: i32,
        level: u16,
        treasure_time: i32,
        evolve_level: i32,
    }
    struct PetDetailInfo {
        flag: i32,
        height: u16,
        weight: u16,
        potential: i32,
        emblem: i32,
        decoration: i32,
        evolve_level: i32,
    }
    struct PetSpawnInfo {
        index: i32,
        monster: i32,
        level: i16,
        _a: i32,
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
        spt: i32,
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
    struct PetRareSpawnInfo {
        index: i32,
        monster: i32,
        level: i16,
        time: u32,
        typ: i8,
    }
    struct PetFreeInfo {
        pid: i32,
        monster: i32,
        level: u16,
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
    struct PetDictionaryInfo {
        monster: i32,
        flag: u8,
    }
    struct PetDictionaryItemInfo {
        item: i32,
        count: i32,
    }
    struct PetDictionaryPetInfo {
        monster: i32,
        pid: i32,
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
            base: PetBaseInfo,
            detail: PetDetailInfo,
            learn: PetLearnInfo,
            _a: i32,
            born: PetBornInfo,
        }
    }
    PetSetFighting {
        Client {
            pid: i32,
        }
        Server {
            second: i32,
            first: i32,
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
            bag: i8,
        }
        Server {
            first: i32,
            pid: i32,
            bag: i8,
            pet: Option<PetInfo>,
        }
    }
    PetPutBagStorage {
        Client {
            pid: i32,
            bag: i8,
        }
        Server {
            pid: i32,
            bag: i8,
            pet: Option<PetInfo>,
        }
    }
    PetPutBagExchange {
        Client {
            bag_pid: i32,
            storage_pid: i32,
        }
        Server {
            bag_pid: i32,
            storage_pid: i32,
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
            pid: i32,
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
    PetRewardStatus {
        Client {}
        Server {
            data: Hex<200>,
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
            pid: i32,
            skills: Vec<i32>,
        }
        Server {
            pid: i32,
            skills: Vec<i32>,
            candidate_skills: Vec<i32>,
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
    PetDictionaryList {
        Client {}
        Server {
            gained: i32,
            pets: Vec<PetDictionaryInfo>,
        }
    }
    PetDictionaryGift {
        Client {}
        Server {
            data: Vec<i32>,
        }
    }
    PetDictionaryReward {
        Client {
            id: i32,
        }
        Server {
            items: Vec<PetDictionaryItemInfo>,
            pets: Vec<PetDictionaryPetInfo>,
        }
    }
}
