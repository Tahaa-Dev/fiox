use csv::ReaderBuilder;

use std::path::PathBuf;

use crate::utils::BetterExpect;

pub fn validate_csv(path: &PathBuf, verbose: bool) {
    let mut reader = ReaderBuilder::new().from_path(path).better_expect(
        format!(
            "ERROR: Couldn't open input file [{}] for validation.",
            path.to_str().unwrap_or("[input.csv]")
        )
        .as_str(),
        verbose,
    );

    // Iterate through records and use BetterExpect to print an error message
    reader.records().enumerate().for_each(|(idx, rec)| {
        rec.better_expect(
            format!(
                "ERROR: Serialization error in input file {} at line [{}]",
                path.to_str().unwrap_or("[input.csv]"),
                idx
            )
            .as_str(),
            verbose,
        );
    });
}
