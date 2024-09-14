use super::*;

cmd_object! {
    struct QuestServerBuffer {
        status: u8,
        extra_data: u8,
    }
    struct QuestClientBuffer {
        status: u8,
        extra_data: [u8;3],
    }
    struct QuestBuffer {
        quest_id: i32,
        server_buffer: [QuestServerBuffer;10],
        client_buffer: [QuestClientBuffer;11],
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
            step_id: u8,
            reward: u8,
            data: [QuestClientBuffer;11],
        }
        Server {
            quest_id: i32,
            step_id: u8,
            reward: ItemReward,
        }
    }
}
