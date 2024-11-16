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
        time: i32,
        lv: i32,
    }
    struct ItemPetInfo {
        monster: i32,
        pid: i32,
    }
    struct ItemReward {
        item_add: Vec<ItemInfo>,
        item_sub: Vec<ItemInfo>,
        pet_add: ItemPetInfo,
    }
    struct ItemPetRideInfo {
        chip: i32,
        pid: i32,
        time: i32,
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
    ItemCoinBuy {
        Client {
            buy_id: i32,
            count: i16,
        }
        Server {
            item: ItemInfo,
            coins: i32,
        }
    }
    ItemMoneyBuy {
        Client {
            buy_id: i32,
            count: i32,
            pass: Hex<16>,
        }
        Server {
            buy_id: i32,
            count: i32,
            out_mi: u32,
            current_mi: u32,
            two_out_mi: u32,
            two_current_mi: u32,
            points: i32,
        }
    }
    ItemBuyMoney {
        Client {
            count: i32,
            pass: Hex<16>,
        }
        Server {
            money: i32,
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
            swap: i32,
            count: i32,
            data: Vec<i32>,
        }
        Server {
            item_sub: Vec<ItemInfo>,
            item_add: Vec<ItemInfo>,
            pet: ItemPetInfo,
            ride: ItemPetRideInfo,
        }
    }
    ItemMeeMoneyCount {
        Client {}
        Server {
            count:i32,
        }
    }
    ItemMoneyCount {
        Client {
            data: Hex<16>
        }
        Server {
            count: i32,
        }
    }
    ItemShopDetails {
        Client {
            id: i32,
        }
        Server {
            item_id: i32,
            price: i32,
            vip_price: i32,
            non_vip_price: i32,
            item_type: u8,
            category: u8,
            gift_money: i32,
            must_vip: u8,
            max_limit: i32,
            total_count: i32,
            current_count: i32,
            is_valid: u8,
            flag: u8,
        }
    }
    ItemMiCipherCheck {
        Client {}
        Server {
            has_cipher: i32,
        }
    }
    ItemEquipEnhance {
        Client {
            equip: Vec<ItemEuqipInfo>,
        }
        Server {
            item: i32,
            equip: i32,
        }
    }
}
