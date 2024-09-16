use crate::models::{Application, ApplicationStatus};
use log::{debug, error};
use rusqlite::{params, Connection};
use std::error::Error;
use chrono::{DateTime, Utc};
use crate::models::application::ApplicationUpdateRequest;

pub fn get_all(
    conn: &mut Connection,
    limit: i64,
    offset: i64,
) -> Result<Vec<Application>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, job_seeker_id, job_id, cover_letter, resume, status, applied_at
         FROM applications LIMIT ?1 OFFSET ?2"
    )?;
    let application_iter = stmt.query_map(params![limit, offset], |row| {
        let applied_at: String = row.get(6)?;

        Ok(Application {
            id: row.get(0)?,
            job_seeker_id: row.get(1)?,
            job_id: row.get(2)?,
            cover_letter: row.get(3)?,
            resume: row.get(4)?,
            status: row.get(5)?,
            applied_at: DateTime::parse_from_rfc3339(&applied_at).unwrap().with_timezone(&Utc),
        })
    })?;

    let mut applications = Vec::new();
    for application in application_iter {
        applications.push(application?);
    }
    Ok(applications)
}

pub fn create(conn: &mut Connection, application: Application) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO applications (job_seeker_id, job_id, cover_letter, resume, status, applied_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            application.job_seeker_id,
            application.job_id,
            application.cover_letter,
            application.resume,
            application.status as i32,
            application.applied_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

pub fn delete(conn: &mut Connection, id: i64) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM applications WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_by_id(conn: &mut Connection, id: i64) -> Result<Option<Application>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, job_seeker_id, job_id, cover_letter, resume, status, applied_at
         FROM applications WHERE id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let applied_at: String = row.get(6)?;

        let application = Application {
            id: row.get(0)?,
            job_seeker_id: row.get(1)?,
            job_id: row.get(2)?,
            cover_letter: row.get(3)?,
            resume: row.get(4)?,
            status: row.get(5)?,
            applied_at: DateTime::parse_from_rfc3339(&applied_at)?.with_timezone(&Utc),
        };
        debug!("APPLICATION: {:#?}", application);
        Ok(Some(application))
    } else {
        error!("APPLICATION NOT FOUND");
        Ok(None)
    }
}

pub fn update(conn: &mut Connection, id: i64, application: ApplicationUpdateRequest) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "UPDATE applications
         SET cover_letter = COALESCE(?1, cover_letter), resume = COALESCE(?2, resume), status = COALESCE(?3, status)
         WHERE id = ?4",
        params![
            application.cover_letter,
            application.resume,
            application.status.map(|s| s as i32),
            id,
        ],
    )?;
    debug!("Application updated in database.");
    Ok(())
}

pub fn get_total_count(conn: &mut Connection) -> Result<i64, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM applications")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}