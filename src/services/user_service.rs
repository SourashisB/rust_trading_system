use crate::models::user::{User, UserRole};
use crate::repositories::user_repository::UserRepository;
use uuid::Uuid;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new() -> Self {
        UserService {
            user_repository: UserRepository::new(),
        }
    }

    pub fn create_user(&self, username: String, password: String, role: UserRole) -> Result<User, String> {
        let password_hash = self.hash_password(&password);
        let user = User::new(username, password_hash, role);
        self.user_repository.add_user(user.clone())?;
        Ok(user)
    }

    pub fn get_user_by_id(&self, id: &Uuid) -> Option<User> {
        self.user_repository.get_user_by_id(id)
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Option<User> {
        let user = self.user_repository.get_user_by_username(username)?;
        if self.verify_password(password, &user.password_hash) {
            Some(user)
        } else {
            None
        }
    }

    fn hash_password(&self, password: &str) -> String {
        // In a real application, use a proper password hashing algorithm
        format!("hashed_{}", password)
    }

    fn verify_password(&self, password: &str, hash: &str) -> bool {
        // In a real application, use a proper password verification method
        hash == &format!("hashed_{}", password)
    }
}