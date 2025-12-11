use std::path::PathBuf;

use serde::de::IgnoredAny;

use crate::utils::BetterExpect;

pub fn toml_reader(path: &PathBuf, verbose: bool) -> Vec<u8> {
    let file_bytes = std::fs::read(path).better_expect(
        format!(
            "ERROR: Couldn't read input TOML file [{}].",
            path.to_str().unwrap_or("[input.toml]")
        )
        .as_str(),
        verbose,
    );

    toml::from_slice::<IgnoredAny>(&file_bytes).better_expect(
        format!(
            "ERROR: Serialization error in input TOML file [{}].",
            path.to_str().unwrap_or("[input.toml]")
        )
        .as_str(),
        verbose,
    );

    file_bytes
}
