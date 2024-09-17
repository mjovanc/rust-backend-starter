use crate::models::Job;
use log::{debug, error};
use rusqlite::{params, Connection};
use std::error::Error;
use chrono::{DateTime, Utc};

pub fn get_all(
    conn: &mut Connection,
    limit: i64,
    offset: i64,
) -> Result<Vec<Job>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, employer_id, title, description, location, salary, employment_type, posted_at, updated_at
         FROM jobs LIMIT ?1 OFFSET ?2"
    )?;
    let job_iter = stmt.query_map(params![limit, offset], |row| {
        let posted_at: String = row.get(7)?;
        let updated_at: String = row.get(8)?;

        Ok(Job {
            id: row.get(0)?,
            employer_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            location: row.get(4)?,
            salary: row.get(5)?,
            employment_type: row.get(6)?,
            posted_at: DateTime::parse_from_rfc3339(&posted_at).unwrap().with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at).unwrap().with_timezone(&Utc),
        })
    })?;

    let mut jobs = Vec::new();
    for job in job_iter {
        jobs.push(job?);
    }
    Ok(jobs)
}

pub fn create(conn: &mut Connection, job: Job) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "INSERT INTO jobs (employer_id, title, description, location, salary, employment_type, posted_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            job.employer_id,
            job.title,
            job.description,
            job.location,
            job.salary,
            job.employment_type as i32,
            job.posted_at.to_rfc3339(),
            job.updated_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

pub fn delete(conn: &mut Connection, id: i64) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM jobs WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_by_id(conn: &mut Connection, id: i64) -> Result<Option<Job>, Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, employer_id, title, description, location, salary, employment_type, posted_at, updated_at
         FROM jobs WHERE id = ?1"
    )?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        let posted_at: String = row.get(7)?;
        let updated_at: String = row.get(8)?;

        let job = Job {
            id: row.get(0)?,
            employer_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            location: row.get(4)?,
            salary: row.get(5)?,
            employment_type: row.get(6)?,
            posted_at: DateTime::parse_from_rfc3339(&posted_at)?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
        };
        debug!("JOB: {:#?}", job);
        Ok(Some(job))
    } else {
        error!("JOB NOT FOUND");
        Ok(None)
    }
}

pub fn update(conn: &mut Connection, id: i64, job: Job) -> Result<(), Box<dyn Error>> {
    conn.execute(
        "UPDATE jobs
         SET employer_id = COALESCE(?1, employer_id), title = COALESCE(?2, title), description = COALESCE(?3, description),
             location = COALESCE(?4, location), salary = COALESCE(?5, salary), employment_type = COALESCE(?6, employment_type),
             updated_at = ?7
         WHERE id = ?8",
        params![
            job.employer_id,
            job.title,
            job.description,
            job.location,
            job.salary,
            job.employment_type as i32,
            Utc::now().to_rfc3339(),
            job.id,
        ],
    )?;
    debug!("Job updated in database.");
    Ok(())
}

pub fn get_total_count(conn: &mut Connection) -> Result<i64, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM jobs")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}