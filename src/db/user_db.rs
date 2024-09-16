use crate::models::{User, UserRole};
use log::{debug, error};
use rusqlite::{params, Connection};
use std::error::Error;
use chrono::{DateTime, Utc};
use crate::models::user::UserUpdateRequest;

pub fn get_all(
    conn: &mut Connection,
    limit: i64,
    offset: i64,
) -> Result<Vec<User>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, email, password, role, created_at, updated_at
         FROM users LIMIT ?1 OFFSET ?2"
    )?;
    let user_iter = stmt.query_map(params![limit, offset], |row| {
        let created_at: String = row.get(5)?;
        let updated_at: String = row.get(6)?;

        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            password: row.get(3)?,
            role: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_at).unwrap().with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at).unwrap().with_timezone(&Utc),
        })
    })?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }
    Ok(users)
}

pub fn create(conn: &mut Connection, user: UserUpdateRequest) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO users (name, email, password, role, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            user.name,
            user.email,
            user.password,
            user.role.unwrap_or(UserRole::JobSeeker) as i32,
            Utc::now().to_rfc3339(),
            Utc::now().to_rfc3339(),
        ],
    )?;
    Ok(())
}

pub fn delete(conn: &mut Connection, id: i64) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_by_id(conn: &mut Connection, id: i64) -> Result<Option<User>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, email, password, role, created_at, updated_at
         FROM users WHERE id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let created_at: String = row.get(5)?;
        let updated_at: String = row.get(6)?;

        let user = User {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            password: row.get(3)?,
            role: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
        };
        debug!("USER: {:#?}", user);
        Ok(Some(user))
    } else {
        error!("USER NOT FOUND");
        Ok(None)
    }
}

pub fn update(conn: &mut Connection, id: i64, user: User) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "UPDATE users
         SET name = COALESCE(?1, name), email = COALESCE(?2, email), password = COALESCE(?3, password),
             role = COALESCE(?4, role), updated_at = ?5
         WHERE id = ?6",
        params![
            user.name,
            user.email,
            user.password,
            user.role,
            Utc::now().to_rfc3339(),
            id,
        ],
    )?;
    debug!("User updated in database.");
    Ok(())
}

pub fn get_total_count(conn: &mut Connection) -> Result<i64, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}