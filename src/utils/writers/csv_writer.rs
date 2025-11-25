use crate::{utilities::UniversalData, utils::BetterExpect};
use colored::Colorize;
use std::{path::PathBuf, process::exit};

pub fn csv_writer(data: &UniversalData, path: &PathBuf, verbose: bool) {
    // Check if it's a table before writing.
    if let UniversalData::Table { headers, rows } = data {
        // Open CSV file for writing
        let mut wtr = csv::Writer::from_path(path)
            .better_expect("ERROR: Failed to open output CSV file.", verbose);

        // Write headers into the file.
        wtr.write_record(headers)
            .better_expect("ERROR: Failed to write CSV file headers.", verbose);

        // Loop for writing rows into the file.
        for row in rows {
            wtr.write_record(row)
                .better_expect("ERROR: Failed to write CSV file rows.", verbose);
        }
        wtr.flush()
            .better_expect("ERROR: Failed to flush final CSV.", verbose);

    // If data is not a table, then panics to prevent broken conversions.
    } else {
        eprintln!(
            "{}",
            "CSV only supports table based file types or files that can be converted into a table."
                .red()
                .bold()
        );
        exit(1);
    }
}
