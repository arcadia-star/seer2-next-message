use super::*;

cmd_object! {
    struct ItemInfo {
        item_id: i32,
        quantity: i16,
        expiry_time: i32,
    }
    struct ItemEuqipInfo {
        id: i32,
        used: i8,
        time: u32,
        lv: i32,
    }
    struct ItemPetInfo {
        monster: u32,
        pid: u32,
    }
}

cmd_object! {
    ItemGetList {
        Client {
            start: i32,
            end: i32,
        }
        Server {
            items: Vec<ItemInfo>,
        }
    }
    ItemBuyViaCoin {
        Client {
            buy_id: i32,
            count: u16,
        }
        Server {
            item: ItemInfo,
            coins: i32,
        }
    }
    ItemBuyViaMiCoin {
        Client {
            buy_id: i32,
            count: u32,
            pass: [u8;16],
        }
        Server {
            item_id: u32,
            item_count: u32,
            out_mi: u32,
            current_mi: u32,
            two_out_mi: u32,
            two_current_mi: u32,
            buy_score: u32,
        }
    }
    ItemBuyMoney {
        Client {
            current_count: u32,
            pass: [u8;16],
        }
        Server {
            money_count: u32,
        }
    }
    ItemEquipList {
        Client {
            start: i32,
            end: i32,
        }
        Server {
            equips: Vec<ItemEuqipInfo>,
        }
    }
    ItemSoldPlain {
        Client {
            buy_id: i32,
            cnt: u16,
        }
        Server {
            item_id: i32,
            sub_cnt: i16,
            coins: i32,
        }
    }
    ItemExchange {
        Client {
            swap: u32,
            count: u32,
            data: Vec<u32>,
        }
        Server {
            item_sub: Vec<ItemInfo>,
            item_add: Vec<ItemInfo>,
            pet_add: ItemPetInfo,
            ride_chip_id: u32,
            pid: u32,
            ride_chip_time: u32,
        }
    }
}
