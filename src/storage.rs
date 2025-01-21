use std::collections::HashMap;
use std::sync::Mutex;

pub struct User {
    pub chat_id: i64,
    pub username: String,
    pub is_active: bool,
}

pub struct Storage {
    users: Mutex<HashMap<i64, User>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_user(&self, chat_id: i64, username: String) {
        let mut users = self.users.lock().unwrap();
        users.insert(
            chat_id,
            User {
                chat_id,
                username,
                is_active: false,
            },
        );
    }
} 