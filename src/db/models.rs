pub struct UserInfo {
    pub user_name: String,
    pub user_id: String,
    pub joined_at: String,
}

impl UserInfo {
    pub fn new() -> Self {
        UserInfo {
            user_name: String::new(),
            user_id: String::new(),
            joined_at: String::new(),
        }
    }
}
