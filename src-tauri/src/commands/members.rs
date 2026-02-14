use serde::Serialize;
use tauri::State;

use crate::error::AppError;
use crate::models::member::{CreateMember, Member, UpdateMember};
use crate::state::AppState;
use crate::validation::pesel;

#[derive(Serialize)]
pub struct PeselValidationResult {
    pub valid: bool,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub fn list_members(
    state: State<AppState>,
    organization_id: i64,
) -> Result<Vec<Member>, AppError> {
    let conn = state.db.get()?;
    Member::list(&conn, organization_id)
}

#[tauri::command]
pub fn get_member(state: State<AppState>, id: i64) -> Result<Member, AppError> {
    let conn = state.db.get()?;
    Member::get(&conn, id)
}

#[tauri::command]
pub fn create_member(
    state: State<AppState>,
    data: CreateMember,
) -> Result<Member, AppError> {
    // Validate PESEL and auto-derive DOB/gender
    let info = pesel::validate(&data.pesel)?;

    if data.first_name.trim().is_empty() {
        return Err(AppError::Validation("Imię jest wymagane".into()));
    }
    if data.last_name.trim().is_empty() {
        return Err(AppError::Validation("Nazwisko jest wymagane".into()));
    }

    let mut data = data;
    data.date_of_birth = info.date_of_birth;
    data.gender = info.gender;

    let conn = state.db.get()?;
    Member::create(&conn, &data)
}

#[tauri::command]
pub fn update_member(
    state: State<AppState>,
    id: i64,
    data: UpdateMember,
) -> Result<Member, AppError> {
    if data.first_name.trim().is_empty() {
        return Err(AppError::Validation("Imię jest wymagane".into()));
    }
    if data.last_name.trim().is_empty() {
        return Err(AppError::Validation("Nazwisko jest wymagane".into()));
    }

    let conn = state.db.get()?;
    Member::update(&conn, id, &data)
}

#[tauri::command]
pub fn delete_member(state: State<AppState>, id: i64) -> Result<(), AppError> {
    let conn = state.db.get()?;
    Member::delete(&conn, id)
}

#[tauri::command]
pub fn validate_pesel(pesel_str: String) -> PeselValidationResult {
    match pesel::validate(&pesel_str) {
        Ok(info) => PeselValidationResult {
            valid: true,
            date_of_birth: Some(info.date_of_birth),
            gender: Some(info.gender),
            error: None,
        },
        Err(e) => PeselValidationResult {
            valid: false,
            date_of_birth: None,
            gender: None,
            error: Some(e.to_string()),
        },
    }
}
