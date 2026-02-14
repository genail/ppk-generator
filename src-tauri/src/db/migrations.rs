use rusqlite::{Connection, params};

const MIGRATIONS: &[(&str, &str)] = &[
    ("001_initial", include_str!("../../migrations/001_initial.sql")),
];

pub fn run(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    )?;

    for (version, sql) in MIGRATIONS {
        let already_applied: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM schema_migrations WHERE version = ?1",
            params![version],
            |row| row.get(0),
        )?;

        if !already_applied {
            conn.execute_batch(sql)?;
            conn.execute(
                "INSERT INTO schema_migrations (version) VALUES (?1)",
                params![version],
            )?;
            log::info!("Applied migration: {}", version);
        }
    }

    Ok(())
}
