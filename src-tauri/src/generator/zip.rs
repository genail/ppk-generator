use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::AppError;

pub struct GeneratedFiles {
    pub zip_bytes: Vec<u8>,
    pub xml_filename: String,
    pub csv_filename: String,
}

pub fn build(xml_content: &str, csv_content: &str) -> Result<GeneratedFiles, AppError> {
    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();

    let xml_filename = format!("SKLADKA_{}.xml", timestamp);
    let csv_filename = format!("SKLADKA_{}.csv", timestamp);

    let mut buf = Vec::new();
    {
        let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buf));
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        zip.start_file(&xml_filename, options)?;
        zip.write_all(xml_content.as_bytes())?;

        zip.start_file(&csv_filename, options)?;
        zip.write_all(csv_content.as_bytes())?;

        zip.finish()?;
    }

    Ok(GeneratedFiles {
        zip_bytes: buf,
        xml_filename,
        csv_filename,
    })
}
