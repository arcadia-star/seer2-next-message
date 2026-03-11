use std::collections::HashMap;

pub struct Secrets {
    pub uid: u32,
    pub password: String,
}

impl Secrets {
    pub fn from_env() -> Self {
        dotenv::from_filename(".secret").ok();
        let map: HashMap<String, String> = dotenv::vars().collect();
        Self {
            uid: map.get("UID").unwrap().parse().unwrap(),
            password: map.get("PASSWORD").unwrap().clone(),
        }
    }
}
