use crate::domain::user::User;
use crate::domain::user::UserId;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserManager {
    users: HashMap<UserId, Arc<User>>,
}

impl UserManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: Arc<User>) {
        self.users.insert(user.user_id, user);
    }

    pub fn remove_user(&mut self, user_id: &UserId) {
        self.users.remove(user_id);
    }
}
