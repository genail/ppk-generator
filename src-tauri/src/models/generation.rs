use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub id: i64,
    pub organization_id: i64,
    pub period_year: i32,
    pub period_month: i32,
    pub generated_at: String,
    pub file_path: String,
    pub total_employee_basic: String,
    pub total_employer_basic: String,
    pub member_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationWithSnapshot {
    pub id: i64,
    pub organization_id: i64,
    pub period_year: i32,
    pub period_month: i32,
    pub generated_at: String,
    pub file_path: String,
    pub total_employee_basic: String,
    pub total_employer_basic: String,
    pub member_count: i32,
    pub snapshot_json: String,
}

impl Generation {
    pub fn list(conn: &Connection, organization_id: i64) -> Result<Vec<Generation>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT id, organization_id, period_year, period_month, generated_at,
                    file_path, total_employee_basic, total_employer_basic, member_count
             FROM generations
             WHERE organization_id = ?1
             ORDER BY generated_at DESC",
        )?;

        let rows = stmt.query_map(params![organization_id], |row| {
            Ok(Generation {
                id: row.get(0)?,
                organization_id: row.get(1)?,
                period_year: row.get(2)?,
                period_month: row.get(3)?,
                generated_at: row.get(4)?,
                file_path: row.get(5)?,
                total_employee_basic: row.get(6)?,
                total_employer_basic: row.get(7)?,
                member_count: row.get(8)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn get_with_snapshot(conn: &Connection, id: i64) -> Result<GenerationWithSnapshot, AppError> {
        conn.query_row(
            "SELECT id, organization_id, period_year, period_month, generated_at,
                    file_path, total_employee_basic, total_employer_basic, member_count,
                    snapshot_json
             FROM generations WHERE id = ?1",
            params![id],
            |row| {
                Ok(GenerationWithSnapshot {
                    id: row.get(0)?,
                    organization_id: row.get(1)?,
                    period_year: row.get(2)?,
                    period_month: row.get(3)?,
                    generated_at: row.get(4)?,
                    file_path: row.get(5)?,
                    total_employee_basic: row.get(6)?,
                    total_employer_basic: row.get(7)?,
                    member_count: row.get(8)?,
                    snapshot_json: row.get(9)?,
                })
            },
        )
        .map_err(|_| AppError::NotFound("Generacja nie znaleziona".into()))
    }

    pub fn insert(
        conn: &Connection,
        organization_id: i64,
        period_year: i32,
        period_month: i32,
        snapshot_json: &str,
        file_path: &str,
        total_employee_basic: &str,
        total_employer_basic: &str,
        member_count: i32,
    ) -> Result<Generation, AppError> {
        conn.execute(
            "INSERT INTO generations (organization_id, period_year, period_month,
                snapshot_json, file_path, total_employee_basic, total_employer_basic, member_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                organization_id,
                period_year,
                period_month,
                snapshot_json,
                file_path,
                total_employee_basic,
                total_employer_basic,
                member_count,
            ],
        )?;

        let id = conn.last_insert_rowid();
        Ok(Generation {
            id,
            organization_id,
            period_year,
            period_month,
            generated_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            file_path: file_path.to_string(),
            total_employee_basic: total_employee_basic.to_string(),
            total_employer_basic: total_employer_basic.to_string(),
            member_count,
        })
    }
}
