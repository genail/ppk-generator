use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Błąd bazy danych: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Błąd puli połączeń: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("Błąd walidacji: {0}")]
    Validation(String),

    #[error("Nie znaleziono: {0}")]
    NotFound(String),

    #[error("Błąd generowania: {0}")]
    Generation(String),

    #[error("Błąd IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("Błąd ZIP: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("{0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
