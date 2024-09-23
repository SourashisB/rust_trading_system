use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use crate::models::user::{User, UserRole};
use std::path::Path;

pub struct UserRepository {
    db_path: Box<Path>,
}

impl UserRepository {
    pub fn new(db_path: &Path) -> Self {
        UserRepository {
            db_path: db_path.into(),
        }
    }

    pub fn add_user(&self, user: &User) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO users (id, username, password_hash, role) VALUES (?1, ?2, ?3, ?4)",
            params![
                user.id.to_string(),
                user.username,
                user.password_hash,
                format!("{:?}", user.role)
            ],
        )?;
        Ok(())
    }

    pub fn get_user_by_id(&self, id: &Uuid) -> Result<Option<User>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?1")?;
        let user_iter = stmt.query_map(params![id.to_string()], |row| {
            Ok(User {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
            })
        })?;

        let user = user_iter.filter_map(Result::ok).next();
        Ok(user)
    }

    pub fn get_user_by_username(&self, username: &str) -> Result<Option<User>, rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?1")?;
        let user_iter = stmt.query_map(params![username], |row| {
            Ok(User {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role: serde_json::from_str(&row.get::<_, String>(3)?).unwrap(),
            })
        })?;

        let user = user_iter.filter_map(Result::ok).next();
        Ok(user)
    }
}