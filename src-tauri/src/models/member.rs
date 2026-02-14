use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: i64,
    pub organization_id: i64,
    pub pesel: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub citizenship: String,
    pub second_name: String,
    pub doc_type: String,
    pub doc_number: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMember {
    pub organization_id: i64,
    pub pesel: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub citizenship: Option<String>,
    pub second_name: Option<String>,
    pub doc_type: Option<String>,
    pub doc_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMember {
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub citizenship: Option<String>,
    pub second_name: Option<String>,
    pub doc_type: Option<String>,
    pub doc_number: Option<String>,
    pub status: Option<String>,
}

const SELECT_COLS: &str = "id, organization_id, pesel, first_name, last_name, gender, date_of_birth, citizenship, second_name, doc_type, doc_number, status, created_at, updated_at";

fn row_to_member(row: &rusqlite::Row) -> rusqlite::Result<Member> {
    Ok(Member {
        id: row.get(0)?,
        organization_id: row.get(1)?,
        pesel: row.get(2)?,
        first_name: row.get(3)?,
        last_name: row.get(4)?,
        gender: row.get(5)?,
        date_of_birth: row.get(6)?,
        citizenship: row.get(7)?,
        second_name: row.get(8)?,
        doc_type: row.get(9)?,
        doc_number: row.get(10)?,
        status: row.get(11)?,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

impl Member {
    pub fn list(conn: &Connection, organization_id: i64) -> Result<Vec<Member>, AppError> {
        let sql = format!(
            "SELECT {} FROM members WHERE organization_id = ?1 ORDER BY last_name, first_name",
            SELECT_COLS
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![organization_id], |row| row_to_member(row))?;

        let mut members = Vec::new();
        for row in rows {
            members.push(row?);
        }
        Ok(members)
    }

    pub fn list_active(conn: &Connection, organization_id: i64) -> Result<Vec<Member>, AppError> {
        let sql = format!(
            "SELECT {} FROM members WHERE organization_id = ?1 AND status = 'active' ORDER BY last_name, first_name",
            SELECT_COLS
        );
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![organization_id], |row| row_to_member(row))?;

        let mut members = Vec::new();
        for row in rows {
            members.push(row?);
        }
        Ok(members)
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Member, AppError> {
        let sql = format!("SELECT {} FROM members WHERE id = ?1", SELECT_COLS);
        conn.query_row(&sql, params![id], |row| row_to_member(row))
            .map_err(|_| AppError::NotFound("Uczestnik nie znaleziony".into()))
    }

    pub fn create(conn: &Connection, data: &CreateMember) -> Result<Member, AppError> {
        conn.execute(
            "INSERT INTO members (organization_id, pesel, first_name, last_name, gender, date_of_birth, citizenship, second_name, doc_type, doc_number)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                data.organization_id,
                data.pesel,
                data.first_name,
                data.last_name,
                data.gender,
                data.date_of_birth,
                data.citizenship.as_deref().unwrap_or("PL"),
                data.second_name.as_deref().unwrap_or(""),
                data.doc_type.as_deref().unwrap_or(""),
                data.doc_number.as_deref().unwrap_or(""),
            ],
        )?;

        let id = conn.last_insert_rowid();
        Self::get(conn, id)
    }

    pub fn update(conn: &Connection, id: i64, data: &UpdateMember) -> Result<Member, AppError> {
        let affected = conn.execute(
            "UPDATE members SET first_name = ?1, last_name = ?2, gender = ?3, date_of_birth = ?4,
             citizenship = ?5, second_name = ?6, doc_type = ?7, doc_number = ?8, status = ?9,
             updated_at = datetime('now') WHERE id = ?10",
            params![
                data.first_name,
                data.last_name,
                data.gender,
                data.date_of_birth,
                data.citizenship.as_deref().unwrap_or("PL"),
                data.second_name.as_deref().unwrap_or(""),
                data.doc_type.as_deref().unwrap_or(""),
                data.doc_number.as_deref().unwrap_or(""),
                data.status.as_deref().unwrap_or("active"),
                id,
            ],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound("Uczestnik nie znaleziony".into()));
        }

        Self::get(conn, id)
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
        let affected = conn.execute("DELETE FROM members WHERE id = ?1", params![id])?;
        if affected == 0 {
            return Err(AppError::NotFound("Uczestnik nie znaleziony".into()));
        }
        Ok(())
    }
}
