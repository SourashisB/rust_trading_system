use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Clone)]
pub struct UserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_user(&self, user: User) -> Result<(), String> {
        let mut users = self.users.lock().unwrap();
        if users.iter().any(|u| u.username == user.username) {
            return Err("Username already exists".to_string());
        }
        users.push(user);
        Ok(())
    }

    pub fn get_user_by_id(&self, id: &Uuid) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.iter().find(|u| u.id == *id).cloned()
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.iter().find(|u| u.username == username).cloned()
    }
}