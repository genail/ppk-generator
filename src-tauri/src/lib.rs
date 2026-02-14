mod commands;
mod db;
pub mod error;
pub mod generator;
pub mod models;
mod state;
pub mod validation;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Set up database
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            let db_path = app_dir.join("ppk_gen.db");

            log::info!("Database path: {:?}", db_path);

            let pool = db::pool::create_pool(&db_path)
                .expect("Failed to create database pool");

            app.manage(AppState { db: pool });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Organizations
            commands::organizations::list_organizations,
            commands::organizations::get_organization,
            commands::organizations::create_organization,
            commands::organizations::update_organization,
            commands::organizations::delete_organization,
            // Members
            commands::members::list_members,
            commands::members::get_member,
            commands::members::create_member,
            commands::members::update_member,
            commands::members::delete_member,
            commands::members::validate_pesel,
            // Contributions
            commands::contributions::list_contributions,
            commands::contributions::upsert_contribution,
            commands::contributions::prefill_contributions,
            commands::contributions::get_available_periods,
            // Generations
            commands::generations::generate_ppk,
            commands::generations::list_generations,
            commands::generations::get_generation,
            commands::generations::export_generation,
            commands::generations::save_zip_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
