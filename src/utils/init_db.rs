use std::env;
use dotenv::dotenv;
use rusqlite::{Connection, Result};

pub fn initialize_database() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn = Connection::open(database_url)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role TEXT CHECK(role IN ('job_seeker', 'employer')) NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS jobs (
            id INTEGER PRIMARY KEY,
            employer_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            location TEXT NOT NULL,
            salary TEXT,
            employment_type TEXT CHECK(employment_type IN ('full_time', 'part_time', 'contract')),
            posted_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (employer_id) REFERENCES User(id)
        );

        CREATE TABLE IF NOT EXISTS application (
            id INTEGER PRIMARY KEY,
            job_seeker_id INTEGER NOT NULL,
            job_id INTEGER NOT NULL,
            cover_letter TEXT,
            resume TEXT,
            status TEXT CHECK(status IN ('pending', 'reviewed', 'accepted', 'rejected')) NOT NULL,
            applied_at TEXT NOT NULL,
            FOREIGN KEY (job_seeker_id) REFERENCES User(id),
            FOREIGN KEY (job_id) REFERENCES Job(id)
        );
        "
    )?;

    // Optionally, insert initial data
    /*conn.execute(
        "INSERT INTO User (id, name, email, password, role, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (1, "John Doe", "john.doe@example.com", "hashed_password_here", "job_seeker", Utc::now().to_rfc3339(), Utc::now().to_rfc3339())
    )?;*/

    Ok(())
}