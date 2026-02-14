use serde::Serialize;
use tauri::State;

use crate::error::AppError;
use crate::models::contribution::{Contribution, ContributionWithMember, UpsertContribution};
use crate::state::AppState;

#[derive(Serialize)]
pub struct Period {
    pub year: i32,
    pub month: i32,
}

#[tauri::command]
pub fn list_contributions(
    state: State<AppState>,
    organization_id: i64,
    year: i32,
    month: i32,
) -> Result<Vec<ContributionWithMember>, AppError> {
    let conn = state.db.get()?;
    Contribution::list_for_period(&conn, organization_id, year, month)
}

#[tauri::command]
pub fn upsert_contribution(
    state: State<AppState>,
    data: UpsertContribution,
) -> Result<(), AppError> {
    let conn = state.db.get()?;
    Contribution::upsert(&conn, &data)
}

#[tauri::command]
pub fn prefill_contributions(
    state: State<AppState>,
    organization_id: i64,
    year: i32,
    month: i32,
) -> Result<i64, AppError> {
    let conn = state.db.get()?;
    Contribution::prefill(&conn, organization_id, year, month)
}

#[tauri::command]
pub fn get_available_periods(
    state: State<AppState>,
    organization_id: i64,
) -> Result<Vec<Period>, AppError> {
    let conn = state.db.get()?;
    let periods = Contribution::get_available_periods(&conn, organization_id)?;
    Ok(periods
        .into_iter()
        .map(|(year, month)| Period { year, month })
        .collect())
}
