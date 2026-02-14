use tauri::State;

use crate::error::AppError;
use crate::models::organization::{CreateOrganization, Organization, UpdateOrganization};
use crate::state::AppState;
use crate::validation::{nip, regon};

#[tauri::command]
pub fn list_organizations(state: State<AppState>) -> Result<Vec<Organization>, AppError> {
    let conn = state.db.get()?;
    Organization::list(&conn)
}

#[tauri::command]
pub fn get_organization(state: State<AppState>, id: i64) -> Result<Organization, AppError> {
    let conn = state.db.get()?;
    Organization::get(&conn, id)
}

#[tauri::command]
pub fn create_organization(
    state: State<AppState>,
    data: CreateOrganization,
) -> Result<Organization, AppError> {
    nip::validate(&data.nip)?;
    regon::validate(&data.regon)?;

    if data.name.trim().is_empty() {
        return Err(AppError::Validation("Nazwa organizacji jest wymagana".into()));
    }

    let conn = state.db.get()?;
    Organization::create(&conn, &data)
}

#[tauri::command]
pub fn update_organization(
    state: State<AppState>,
    id: i64,
    data: UpdateOrganization,
) -> Result<Organization, AppError> {
    nip::validate(&data.nip)?;
    regon::validate(&data.regon)?;

    if data.name.trim().is_empty() {
        return Err(AppError::Validation("Nazwa organizacji jest wymagana".into()));
    }

    let conn = state.db.get()?;
    Organization::update(&conn, id, &data)
}

#[tauri::command]
pub fn delete_organization(state: State<AppState>, id: i64) -> Result<(), AppError> {
    let conn = state.db.get()?;
    Organization::delete(&conn, id)
}
