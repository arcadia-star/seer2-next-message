use super::*;

cmd_object! {
    struct FightPetInfo {
        pid: i32,
        anger: i32,
        monster: i32,
        position: u8,
        level: i16,
        hp: i32,
        hp_max: i32,
        skills: Vec<i32>,
        evolve_level: i32,
    }
    struct FightUserInfo {
        uid: i32,
        nick: CString<16>,
        pets: Vec<FightPetInfo>,
        pets_change: Vec<FightPetInfo>,
    }
    struct FightTeamInfo {
        side: u8,
        leader: i32,
        users: Vec<FightUserInfo>,
    }
    struct FightBuffInfo {
        buff: i32,
        round: i32,
        dummy0: i32,
        dummy1: i32,
        dummy2: i32,
    }
    struct FightHurtInfo {
        attack: u8,
        uid: i32,
        pid: i32,
        skill: i32,
        position: u8,
        hp: i32,
        hpm: i32,
        anger: i16,
        dying: u8,
        atk: u8,
        def: u8,
        spk: u8,
        spf: u8,
        spd: u8,
        buff: Vec<FightBuffInfo>,
    }
    struct FighterRevenueInfo {
        pid: u32,
        sex: u8,
        level: u16,
        hp: u32,
        hp_max: u32,
        atk: u16,
        def: u16,
        sp_atk: u16,
        sp_def: u16,
        spd: u16,
        level_up_needed_exp: u32,
        character: u16,
        gained_exp: u32,
        skills: Vec<u32>,
    }
    struct FightBuffResultInfo {
        state_id: u32,
        change_hp: i32,
    }
    struct FightPetAngerInfo {
        uid: i32,
        pid: i32,
        anger: i32,
    }
    struct FightPetHpInfo {
        uid: i32,
        pid: i32,
        hp: i32,
    }
    struct FightPetMorphInfo {
        uid: u32,
        dying: u8,
        monster: u32,
        pid: u32,
    }
    struct FightPetResultInfo {
        base: PetBaseInfo,
        skill: Vec<u32>,
        gained_skill: Vec<u32>,
        point_unused: u16,
        two_exp: u32,
        three_exp: u32,
        two_study: u32,
    }
    struct FightItemResultInfo {
        item: u32,
        count: u32,
    }
}

cmd_object! {
   FightLoadResource {
        Client {}
        Server {
            mode: u8,
            team1: FightTeamInfo,
            team2: FightTeamInfo,
            weather: u8,
            catchable: u8,
        }
    }
    FightTurnStartNotify {
        Server {}
    }
    FightHurtNotify {
        Server {
            res: Vec<FightHurtInfo>,
            notify_index: i32,
            critical: i32,
            skill_type_delation: i32,
            atk_times: i32,
            changed_hp: i32,
        }
    }
    FightLoadMapNotify {
        Server {}
    }
    FightRevenueNotify {
        Server {
            revenue: Vec<FighterRevenueInfo>,
            exp_rate: u32,
        }
    }
    FightTurnNextNotify {
        Server {
            turn: u8,
            weather: u8,
        }
    }
    FightPetChangedNotify {
        Server {
            uid: u32,
            pid: u32,
            anger: u32,
            buff: Vec<FightBuffInfo>,
        }
    }
    FightBuffResultNotify {
        Server {
            uid: u32,
            pid: u32,
            dying: u8,
            result: Vec<FightBuffResultInfo>,
        }
    }
    FightItemUseNotify {
        Server {
            side: u8,
            uid: u32,
            pid: u32,
            skill: u32,
            position: u8,
            hp: u32,
            hp_max: u32,
            anger: u16,
            dying: u8,
            atk: u8,
            def: u8,
            sp_atk: u8,
            sp_def: u8,
            spd: u8,
            buff: Vec<FightBuffInfo>,
        }
    }
    FightFeatureResultNotify {
        Server {
            uid: u32,
            pid: u32,
            hp: u32,
            t1: u8,
        }
    }
    FightEscapeNotify {
        Server {
            uid: i32,
        }
    }
    FightPetPositionNotify {
        Server {
            left: Vec<FightPetAngerInfo>,
            right: Vec<FightPetAngerInfo>,
        }
    }
    FightPetAngerNotify {
        Server {
            left: FightPetAngerInfo,
            right: FightPetAngerInfo,
        }
    }
    FightPetPositionPvpNotify {
        Server {
            left: Vec<FightPetAngerInfo>,
            right: Vec<FightPetAngerInfo>,
        }
    }
    FightPetFitNotify {
        Server {
            pets: Vec<FightPetHpInfo>,
            skills: Vec<u32>,
        }
    }
    FightPetMorphNotify {
        Server {
            pets: Vec<FightPetMorphInfo>,
        }
    }
    FightCmdCatch {
        Client {
            item: i32,
        }
        Server {
            success: i32,
            pid: i32,
        }
    }
    FightCmdChange {
        Client {
            pid: i32,
        }
        Server {
            _a: u8,
        }
    }
    FightCmdMedicine {
        Client {
            pid: i32,
            item: i32,
            count: i32,
        }
        Server {
            item: i32,
            pid: i32,
            hp: i32,
        }
    }
    FightCmdSkill {
        Client {
            skill: i32,
        }
        Server {
            pid: i32,
            skill: i32,
        }
    }
    FightCmdEscape {
        Client {}
        Server {}
    }
    FightEndNotify {
        Server {
            reason: u8,
            winner: u8,
            pets: Vec<FightPetResultInfo>,
            gained_emblem_pid: u32,
            gained_emblem_id: u32,
            gained_item: Vec<FightItemResultInfo>,
        }
    }
    FightBoss {
        Client {
            boss_id: u32,
            data: Vec<u32>,
            single: u8,
            pids: Vec<u32>,
        }
        Server {}
    }
    FightWild {
        Client {
            index: i32,
        }
        Server {}
    }
    FightNpc {
        Client {
            index: u8,
            data: Vec<u32>,
        }
        Server {}
    }
    FightVerifyNotify {
        Server {
            side: u32,
            data: Vec<u8>,
        }
    }
    FightClientVerify {
        Client {}
        Server {}
    }
    FightResReady {
        Client {
            index: u32,
        }
        Server {}
    }
    FightPlayerInvite {
        Client {
            uid: u32,
            mode: u32,
            _a: u8,
        }
        Server {}
    }
    FightPlayerAccept {
        Client {
            ring_id: u32,
            _a: u32,
        }
        Server {}
    }
    FightPlayerCancel {
        Client {}
        Server {}
    }
    FightPvpInvite {
        Client {
            uid: u32,
            mode: u32,
            fight_type: u8,
            pets: Vec<u32>,
        }
        Server {}
    }
    FightPvpAccpet {
        Client {
            uid: u32,
            accept: u32,
            pets: Vec<u32>,
        }
        Server {}
    }
    FightPvpCancel {
        Client {}
        Server {}
    }
    FightPvpAccpetNotify {
        Client {}
        Server {}
    }

}
