use serde::de::IgnoredAny;
use serde_json::Deserializer;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::utils::BetterExpect;

pub fn validate_json(path: &PathBuf, verbose: bool) {
    let file: File = File::open(path).better_expect(
        format!(
            "ERROR: Couldn't open input file [{}] for validation.",
            path.to_str().unwrap_or("[input.json]")
        )
        .as_str(),
        verbose,
    );

    let reader = BufReader::with_capacity(16384, file);

    // Deserialization stream for validation
    let file_stream = Deserializer::from_reader(reader).into_iter::<IgnoredAny>();

    // Iter loop for checking line-by-line
    file_stream.for_each(|item| {
        item.better_expect(
            format!(
                "ERROR: Serialization error in input file [{}].",
                path.to_str().unwrap_or("[input.json]")
            )
            .as_str(),
            verbose,
        );
    });
}
