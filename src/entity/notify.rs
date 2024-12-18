use super::*;

cmd_object! {
    NotifyServerClose {
        Server {}
    }
    NotifyBuddyAdd {
        Server {
            uid:i32,
            nick:CString<16>,
        }
    }
    NotifyBuddyReply {
       Server {
            uid:i32,
            nick:CString<16>,
            black:i8,
        }
    }
    NotifyItemGive {
        Server {
            cmd: u32,
            item_add: Vec<ItemInfo>,
            pet_add: Vec<ItemPetInfo>,
            item_sub: Vec<ItemInfo>,
        }
    }
    NotifyPetSpawn {
        Server {
            pets: Vec<PetSpawnInfo>,
        }
    }
    NotifyPetItemUpdate {
        Server {
            data: Vec<PetItemUpdateInfo>,
        }
    }
    NotifySyncServerTime {
        Server {
            time: u32,
        }
    }
    NotifyPetRareSpawn {
        Server {
            pets: Vec<PetRareSpawnInfo>,
        }
    }
    NotifyBeatCaptain {
        Server {
            typ: u32,
            uid: u32,
            name: CString<16>,
            _a: Vec<u32>,
            _b: Hex<16>,
        }
    }
    NotifyServerEvent {
        Server {
            event_type: i32,
            event_data: Vec<i32>,
        }
    }
    NotifyPetDelete {
        Server {
            pids: Vec<i32>,
        }
    }
}
