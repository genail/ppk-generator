use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

use crate::db::migrations;
use crate::state::DbPool;

pub fn create_pool(db_path: &Path) -> Result<DbPool, Box<dyn std::error::Error>> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::builder().max_size(4).build(manager)?;

    // Configure SQLite
    {
        let conn = pool.get()?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA foreign_keys=ON;
             PRAGMA busy_timeout=5000;",
        )?;
    }

    // Run migrations
    {
        let conn = pool.get()?;
        migrations::run(&conn)?;
    }

    Ok(pool)
}
