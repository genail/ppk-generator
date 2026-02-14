use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionWithMember {
    pub id: i64,
    pub member_id: i64,
    pub period_year: i32,
    pub period_month: i32,
    pub employee_basic: String,
    pub employee_additional: String,
    pub employer_basic: String,
    pub employer_additional: String,
    pub reduced_basic_flag: String,
    pub source: String,
    pub updated_at: String,
    // Member fields
    pub pesel: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub date_of_birth: String,
    pub citizenship: String,
    pub second_name: String,
    pub doc_type: String,
    pub doc_number: String,
    pub member_status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpsertContribution {
    pub member_id: i64,
    pub period_year: i32,
    pub period_month: i32,
    pub employee_basic: Option<String>,
    pub employee_additional: Option<String>,
    pub employer_basic: Option<String>,
    pub employer_additional: Option<String>,
    pub reduced_basic_flag: Option<String>,
}

pub struct Contribution;

impl Contribution {
    pub fn list_for_period(
        conn: &Connection,
        organization_id: i64,
        year: i32,
        month: i32,
    ) -> Result<Vec<ContributionWithMember>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT c.id, c.member_id, c.period_year, c.period_month,
                    c.employee_basic, c.employee_additional, c.employer_basic, c.employer_additional,
                    c.reduced_basic_flag, c.source, c.updated_at,
                    m.pesel, m.first_name, m.last_name, m.gender, m.date_of_birth,
                    m.citizenship, m.second_name, m.doc_type, m.doc_number, m.status
             FROM contributions c
             JOIN members m ON c.member_id = m.id
             WHERE m.organization_id = ?1 AND c.period_year = ?2 AND c.period_month = ?3
             ORDER BY m.last_name, m.first_name",
        )?;

        let rows = stmt.query_map(params![organization_id, year, month], |row| {
            Ok(ContributionWithMember {
                id: row.get(0)?,
                member_id: row.get(1)?,
                period_year: row.get(2)?,
                period_month: row.get(3)?,
                employee_basic: row.get(4)?,
                employee_additional: row.get(5)?,
                employer_basic: row.get(6)?,
                employer_additional: row.get(7)?,
                reduced_basic_flag: row.get(8)?,
                source: row.get(9)?,
                updated_at: row.get(10)?,
                pesel: row.get(11)?,
                first_name: row.get(12)?,
                last_name: row.get(13)?,
                gender: row.get(14)?,
                date_of_birth: row.get(15)?,
                citizenship: row.get(16)?,
                second_name: row.get(17)?,
                doc_type: row.get(18)?,
                doc_number: row.get(19)?,
                member_status: row.get(20)?,
            })
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }

    pub fn upsert(conn: &Connection, data: &UpsertContribution) -> Result<(), AppError> {
        // Validate money amounts
        for (field, name) in [
            (&data.employee_basic, "Składka podstawowa pracownika"),
            (&data.employee_additional, "Składka dodatkowa pracownika"),
            (&data.employer_basic, "Składka podstawowa pracodawcy"),
            (&data.employer_additional, "Składka dodatkowa pracodawcy"),
        ] {
            if let Some(val) = field {
                validate_money(val, name)?;
            }
        }

        conn.execute(
            "INSERT INTO contributions (member_id, period_year, period_month,
                employee_basic, employee_additional, employer_basic, employer_additional,
                reduced_basic_flag, source)
             VALUES (?1, ?2, ?3,
                COALESCE(?4, '0.00'), COALESCE(?5, '0.00'), COALESCE(?6, '0.00'), COALESCE(?7, '0.00'),
                COALESCE(?8, 'N'), 'manual')
             ON CONFLICT(member_id, period_year, period_month)
             DO UPDATE SET
                employee_basic = COALESCE(?4, employee_basic),
                employee_additional = COALESCE(?5, employee_additional),
                employer_basic = COALESCE(?6, employer_basic),
                employer_additional = COALESCE(?7, employer_additional),
                reduced_basic_flag = COALESCE(?8, reduced_basic_flag),
                source = 'manual',
                updated_at = datetime('now')",
            params![
                data.member_id,
                data.period_year,
                data.period_month,
                data.employee_basic,
                data.employee_additional,
                data.employer_basic,
                data.employer_additional,
                data.reduced_basic_flag,
            ],
        )?;

        Ok(())
    }

    pub fn prefill(
        conn: &Connection,
        organization_id: i64,
        target_year: i32,
        target_month: i32,
    ) -> Result<i64, AppError> {
        // For each active member without a contribution for the target period,
        // copy from their most recent existing contribution.
        let copied = conn.execute(
            "INSERT INTO contributions (member_id, period_year, period_month,
                employee_basic, employee_additional, employer_basic, employer_additional,
                reduced_basic_flag, source)
             SELECT m.id, ?2, ?3,
                    prev.employee_basic, prev.employee_additional,
                    prev.employer_basic, prev.employer_additional,
                    prev.reduced_basic_flag, 'prefilled'
             FROM members m
             INNER JOIN (
                 SELECT c.*, ROW_NUMBER() OVER (
                     PARTITION BY c.member_id
                     ORDER BY c.period_year DESC, c.period_month DESC
                 ) AS rn
                 FROM contributions c
                 INNER JOIN members m2 ON c.member_id = m2.id
                 WHERE m2.organization_id = ?1
             ) prev ON prev.member_id = m.id AND prev.rn = 1
             WHERE m.organization_id = ?1
               AND m.status = 'active'
               AND NOT EXISTS (
                   SELECT 1 FROM contributions c2
                   WHERE c2.member_id = m.id
                     AND c2.period_year = ?2
                     AND c2.period_month = ?3
               )",
            params![organization_id, target_year, target_month],
        )?;

        // Create zero-value contributions for active members with no history at all
        let zeroed = conn.execute(
            "INSERT INTO contributions (member_id, period_year, period_month,
                employee_basic, employee_additional, employer_basic, employer_additional,
                reduced_basic_flag, source)
             SELECT m.id, ?2, ?3,
                    '0.00', '0.00', '0.00', '0.00', 'N', 'prefilled'
             FROM members m
             WHERE m.organization_id = ?1
               AND m.status = 'active'
               AND NOT EXISTS (
                   SELECT 1 FROM contributions c
                   WHERE c.member_id = m.id
               )",
            params![organization_id, target_year, target_month],
        )?;

        Ok((copied + zeroed) as i64)
    }

    pub fn get_available_periods(
        conn: &Connection,
        organization_id: i64,
    ) -> Result<Vec<(i32, i32)>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT c.period_year, c.period_month
             FROM contributions c
             JOIN members m ON c.member_id = m.id
             WHERE m.organization_id = ?1
             ORDER BY c.period_year DESC, c.period_month DESC",
        )?;

        let rows = stmt.query_map(params![organization_id], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
        })?;

        let mut periods = Vec::new();
        for row in rows {
            periods.push(row?);
        }
        Ok(periods)
    }
}

fn validate_money(value: &str, field_name: &str) -> Result<(), AppError> {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let dec = Decimal::from_str(value).map_err(|_| {
        AppError::Validation(format!("{}: nieprawidłowa kwota '{}'", field_name, value))
    })?;

    if dec < Decimal::ZERO {
        return Err(AppError::Validation(format!(
            "{}: kwota nie może być ujemna",
            field_name
        )));
    }

    if dec.scale() > 2 {
        return Err(AppError::Validation(format!(
            "{}: kwota może mieć maksymalnie 2 miejsca po przecinku",
            field_name
        )));
    }

    Ok(())
}
