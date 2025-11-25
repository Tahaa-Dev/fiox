use std::path::PathBuf;

use crate::{utilities::UniversalData, utils::BetterExpect};

pub fn csv_reader(path: &PathBuf, verbose: bool) -> UniversalData {
    // Reads into enum type UniversalData::Table
    let mut read =
        csv::Reader::from_path(path).better_expect("ERROR: Failed to read input file.", verbose);

    // file headers
    let headers: Vec<String> = read
        .headers()
        .better_expect("ERROR: Failed to read headers. Make sure CSV file has headers for conversions to work.", verbose)
        .iter()
        .map(|h| h.to_string())
        .collect();

    // file rows
    let rows: Vec<Vec<String>> = read
        .records()
        .map(|r| {
            r.unwrap_or_default()
                .iter()
                .map(|s| s.to_string())
                .collect()
        })
        .collect();

    UniversalData::Table { headers, rows }
}
