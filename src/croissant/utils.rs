//! Utility functions for file operations and CSV processing

use crate::croissant::errors::{Error, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Calculate the SHA-256 hash of a file
pub fn calculate_sha256(file_path: &Path) -> Result<String> {
    let file = File::open(file_path).map_err(|_| Error::file_not_found(file_path))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
}

/// Get CSV column headers and optionally the first data row
pub fn get_csv_columns(csv_path: &Path) -> Result<(Vec<String>, Option<Vec<String>>)> {
    let file = File::open(csv_path).map_err(|_| Error::file_not_found(csv_path))?;
    let mut reader = csv::Reader::from_reader(file);

    // Read headers
    let headers = reader
        .headers()?
        .iter()
        .map(|h| h.trim().to_string())
        .collect::<Vec<String>>();

    // Try to read the first data row for type inference
    let first_row = if let Some(result) = reader.records().next() {
        let record = result?;
        Some(
            record
                .iter()
                .map(|field| field.trim().to_string())
                .collect(),
        )
    } else {
        None
    };

    Ok((headers, first_row))
}

/// Validate if the given path is a valid output file path
pub fn validate_output_path(output_path: &Path) -> Result<()> {
    // Check if the parent directory exists or can be created
    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::invalid_output_path(output_path, format!("Cannot create directory: {}", e))
            })?;
        }
    }

    // Check if we can write to the file by creating a temporary file
    let temp_path = output_path.with_extension("tmp");
    match File::create(&temp_path) {
        Ok(_) => {
            // Clean up the temporary file
            let _ = std::fs::remove_file(&temp_path);
            Ok(())
        }
        Err(e) => Err(Error::invalid_output_path(
            output_path,
            format!("Cannot write to path: {}", e),
        )),
    }
}

/// Clean and normalize a file path
pub fn normalize_path(path: &Path) -> Result<std::path::PathBuf> {
    path_clean::clean(path)
        .canonicalize()
        .map_err(|e| Error::invalid_format(format!("Invalid path: {}", e)))
}

/// Get file size in a human-readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Check if a file exists and is readable
pub fn is_file_readable(path: &Path) -> bool {
    path.exists() && path.is_file() && File::open(path).is_ok()
}

/// Extract file extension from path
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
}

/// Validate CSV file format by attempting to read headers
pub fn validate_csv_format(csv_path: &Path) -> Result<()> {
    let file = File::open(csv_path).map_err(|_| Error::file_not_found(csv_path))?;
    let mut reader = csv::Reader::from_reader(file);

    // Try to read headers
    let headers = reader.headers()?;
    if headers.is_empty() {
        return Err(Error::invalid_format("CSV file has no headers".to_string()));
    }

    // Check for duplicate headers
    let mut seen_headers = std::collections::HashSet::new();
    for header in headers.iter() {
        let trimmed = header.trim();
        if trimmed.is_empty() {
            return Err(Error::invalid_format(
                "CSV file has empty header".to_string(),
            ));
        }
        if !seen_headers.insert(trimmed.to_lowercase()) {
            return Err(Error::invalid_format(format!(
                "CSV file has duplicate header: {}",
                trimmed
            )));
        }
    }

    Ok(())
}
