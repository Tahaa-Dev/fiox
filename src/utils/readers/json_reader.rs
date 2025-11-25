use std::path::PathBuf;

use crate::{
    utilities::{UniversalData, Vals},
    utils::BetterExpect,
};
use serde_json::Value as JsonVal;

pub fn json_reader(path: &PathBuf, verbose: bool) -> UniversalData {
    // Reads then converts to TOML format
    let content =
        std::fs::read_to_string(path).better_expect("ERROR: Failed to read input file.", verbose);

    let json_des: JsonVal =
        serde_json::from_str(&content).better_expect("ERROR: Failed to deserialize file.", verbose);
    UniversalData::Structured(Vals::Json(json_des))
}
