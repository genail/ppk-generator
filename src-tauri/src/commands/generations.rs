use serde::Serialize;
use tauri::State;

use crate::error::AppError;
use crate::generator;
use crate::models::contribution::Contribution;
use crate::models::generation::{Generation, GenerationWithSnapshot};
use crate::models::organization::Organization;
use crate::state::AppState;

#[derive(Serialize)]
pub struct GenerateResult {
    pub generation: Generation,
    pub zip_bytes: Vec<u8>,
    pub total_employee_basic: String,
    pub total_employee_additional: String,
    pub total_employer_basic: String,
    pub total_employer_additional: String,
    pub member_count: i32,
}

#[tauri::command]
pub fn generate_ppk(
    state: State<AppState>,
    organization_id: i64,
    year: i32,
    month: i32,
) -> Result<GenerateResult, AppError> {
    let conn = state.db.get()?;

    let org = Organization::get(&conn, organization_id)?;
    let contributions = Contribution::list_for_period(&conn, organization_id, year, month)?;

    if contributions.is_empty() {
        return Err(AppError::Generation(
            "Brak składek do wygenerowania dla wybranego okresu".into(),
        ));
    }

    // Calculate totals
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let mut total_emp_basic = Decimal::ZERO;
    let mut total_emp_additional = Decimal::ZERO;
    let mut total_er_basic = Decimal::ZERO;
    let mut total_er_additional = Decimal::ZERO;

    for c in &contributions {
        total_emp_basic += Decimal::from_str(&c.employee_basic).unwrap_or(Decimal::ZERO);
        total_emp_additional += Decimal::from_str(&c.employee_additional).unwrap_or(Decimal::ZERO);
        total_er_basic += Decimal::from_str(&c.employer_basic).unwrap_or(Decimal::ZERO);
        total_er_additional += Decimal::from_str(&c.employer_additional).unwrap_or(Decimal::ZERO);
    }

    let member_count = contributions.len() as i32;

    // Build files
    let xml = generator::xml::build(&org, &contributions, year, month);
    let csv = generator::csv::build(&contributions, year, month);

    let generated = generator::zip::build(&xml, &csv)?;

    // Build snapshot
    let snapshot = serde_json::json!({
        "organization": org,
        "contributions": contributions,
        "period": { "year": year, "month": month },
    });

    // Save generation record
    let generation = Generation::insert(
        &conn,
        organization_id,
        year,
        month,
        &snapshot.to_string(),
        &format!("SKLADKA_{}.zip", chrono::Local::now().format("%Y%m%d_%H%M%S")),
        &format!("{:.2}", total_emp_basic),
        &format!("{:.2}", total_er_basic),
        member_count,
    )?;

    Ok(GenerateResult {
        generation,
        zip_bytes: generated.zip_bytes,
        total_employee_basic: format!("{:.2}", total_emp_basic),
        total_employee_additional: format!("{:.2}", total_emp_additional),
        total_employer_basic: format!("{:.2}", total_er_basic),
        total_employer_additional: format!("{:.2}", total_er_additional),
        member_count,
    })
}

#[tauri::command]
pub fn list_generations(
    state: State<AppState>,
    organization_id: i64,
) -> Result<Vec<Generation>, AppError> {
    let conn = state.db.get()?;
    Generation::list(&conn, organization_id)
}

#[tauri::command]
pub fn get_generation(
    state: State<AppState>,
    id: i64,
) -> Result<GenerationWithSnapshot, AppError> {
    let conn = state.db.get()?;
    Generation::get_with_snapshot(&conn, id)
}

#[tauri::command]
pub fn export_generation(
    state: State<AppState>,
    id: i64,
) -> Result<GenerateResult, AppError> {
    let conn = state.db.get()?;
    let gen = Generation::get_with_snapshot(&conn, id)?;

    let snapshot: serde_json::Value = serde_json::from_str(&gen.snapshot_json)
        .map_err(|e| AppError::Other(format!("Błąd parsowania snapshot: {}", e)))?;

    let org: crate::models::organization::Organization =
        serde_json::from_value(snapshot["organization"].clone())
            .map_err(|e| AppError::Other(format!("Błąd parsowania organizacji: {}", e)))?;

    let contributions: Vec<crate::models::contribution::ContributionWithMember> =
        serde_json::from_value(snapshot["contributions"].clone())
            .map_err(|e| AppError::Other(format!("Błąd parsowania składek: {}", e)))?;

    let xml = generator::xml::build(&org, &contributions, gen.period_year, gen.period_month);
    let csv = generator::csv::build(&contributions, gen.period_year, gen.period_month);
    let generated = generator::zip::build(&xml, &csv)?;

    Ok(GenerateResult {
        generation: Generation {
            id: gen.id,
            organization_id: gen.organization_id,
            period_year: gen.period_year,
            period_month: gen.period_month,
            generated_at: gen.generated_at,
            file_path: gen.file_path,
            total_employee_basic: gen.total_employee_basic.clone(),
            total_employer_basic: gen.total_employer_basic.clone(),
            member_count: gen.member_count,
        },
        zip_bytes: generated.zip_bytes,
        total_employee_basic: gen.total_employee_basic,
        total_employee_additional: "0.00".to_string(),
        total_employer_basic: gen.total_employer_basic,
        total_employer_additional: "0.00".to_string(),
        member_count: gen.member_count,
    })
}

/// Save ZIP bytes to a given file path.
/// The frontend opens the save dialog and passes the chosen path.
#[tauri::command]
pub fn save_zip_file(zip_bytes: Vec<u8>, path: String) -> Result<(), AppError> {
    use std::io::Write;
    let mut file = std::fs::File::create(&path)?;
    file.write_all(&zip_bytes)?;
    Ok(())
}
