use super::*;

cmd_object! {
    NotifyServerClose {
        Server {}
    }
    NotifyBuddyAdd {
        Server {
            uid:i32,
            nick:[u8;16],
        }
    }
    NotifyBuddyReply {
       Server {
            uid:i32,
            nick:[u8;16],
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
    NotifyPetSpawnLobby {
        Server {
            pets: Vec<PetSpawnLobbyInfo>,
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
    NotifyMapRarePet {
        Server {
            pets: Vec<PetMapRareInfo>,
        }
    }
    NotifyBeatCaptain {
        Server {
            typ: u32,
            uid: u32,
            name: [u8; 16],
            _a: Vec<u32>,
            _b: [u32;4],
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
