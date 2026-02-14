use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub nip: String,
    pub regon: String,
    pub contact_person: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganization {
    pub name: String,
    pub nip: String,
    pub regon: String,
    pub contact_person: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganization {
    pub name: String,
    pub nip: String,
    pub regon: String,
    pub contact_person: String,
}

impl Organization {
    pub fn list(conn: &Connection) -> Result<Vec<Organization>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id, name, nip, regon, contact_person, created_at, updated_at
             FROM organizations ORDER BY name",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Organization {
                id: row.get(0)?,
                name: row.get(1)?,
                nip: row.get(2)?,
                regon: row.get(3)?,
                contact_person: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;

        let mut orgs = Vec::new();
        for row in rows {
            orgs.push(row?);
        }
        Ok(orgs)
    }

    pub fn get(conn: &Connection, id: i64) -> Result<Organization, AppError> {
        conn.query_row(
            "SELECT id, name, nip, regon, contact_person, created_at, updated_at
             FROM organizations WHERE id = ?1",
            params![id],
            |row| {
                Ok(Organization {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    nip: row.get(2)?,
                    regon: row.get(3)?,
                    contact_person: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            },
        )
        .map_err(|_| AppError::NotFound("Organizacja nie znaleziona".into()))
    }

    pub fn create(conn: &Connection, data: &CreateOrganization) -> Result<Organization, AppError> {
        conn.execute(
            "INSERT INTO organizations (name, nip, regon, contact_person)
             VALUES (?1, ?2, ?3, ?4)",
            params![data.name, data.nip, data.regon, data.contact_person],
        )?;

        let id = conn.last_insert_rowid();
        Self::get(conn, id)
    }

    pub fn update(conn: &Connection, id: i64, data: &UpdateOrganization) -> Result<Organization, AppError> {
        let affected = conn.execute(
            "UPDATE organizations SET name = ?1, nip = ?2, regon = ?3, contact_person = ?4,
             updated_at = datetime('now') WHERE id = ?5",
            params![data.name, data.nip, data.regon, data.contact_person, id],
        )?;

        if affected == 0 {
            return Err(AppError::NotFound("Organizacja nie znaleziona".into()));
        }

        Self::get(conn, id)
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<(), AppError> {
        let affected = conn.execute("DELETE FROM organizations WHERE id = ?1", params![id])?;
        if affected == 0 {
            return Err(AppError::NotFound("Organizacja nie znaleziona".into()));
        }
        Ok(())
    }
}
