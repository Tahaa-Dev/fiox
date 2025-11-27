use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::utils::BetterExpect;

pub fn validate_ndjson(path: &PathBuf, verbose: bool) {
    let file = File::open(path).better_expect(
        format!(
            "ERROR: Couldn't open input file [{}] for validation.",
            path.to_str().unwrap_or("[input.ndjson]")
        )
        .as_str(),
        verbose,
    );

    let reader = BufReader::with_capacity(16384, file);

    // read lines one by one and deserialize them to check for errors
    reader.lines().enumerate().for_each(|(idx, line)| {
        serde_json::from_str::<serde_json::Value>(
            line.better_expect(
                format!("ERROR: Couldn't read line {}", idx + 1).as_str(),
                verbose,
            )
            .as_str(),
        )
        .better_expect(
            format!(
                "ERROR: Serialization error in input file [{}] at line [{}]",
                path.to_str().unwrap_or("[input.ndjson]"),
                idx + 1
            )
            .as_str(),
            verbose,
        );
    });
}
