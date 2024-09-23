use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Admin,
}

impl User {
    pub fn new(username: String, password_hash: String, role: UserRole) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
            password_hash,
            role,
        }
    }
}