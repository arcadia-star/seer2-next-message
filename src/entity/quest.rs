use super::*;

cmd_object! {
    struct QuestBuffer {
        quest_id: i32,
        buffer: [i64; 8],
    }
}

cmd_object! {
    QuestAccept {
        Client {
            quest_id: i32,
        }
        Server {
            quest_id: i32,
        }
    }
    QuestAbort {
        Client {
            quest_id: i32,
        }
        Server {
            quest_id: i32,
        }
    }
    QuestGetBuffer {
        Client {
            quest_ids: Vec<i32>,
        }
        Server {
            buffer: Vec<QuestBuffer>,
        }
    }
    QuestSubmit {
        Client {
            quest_id: i32,
            step: u8,
            flag: u8,
            data0: [u8; 11],
            data1: [u8; 11],
            data2: [u8; 11],
        }
        Server {
            quest_id: i32,
            step: u8,
            item_add: Vec<ItemInfo>,
            item_sub: Vec<ItemInfo>,
            pet_add: ItemPetInfo,
        }
    }
}
