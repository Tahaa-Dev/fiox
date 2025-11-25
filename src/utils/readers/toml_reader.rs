use std::path::PathBuf;

use crate::{
    utilities::{UniversalData, Vals},
    utils::BetterExpect,
};
use toml::Value as TomlVal;

pub fn toml_reader(path: &PathBuf, verbose: bool) -> UniversalData {
    // reads file and then formats to JSON since I still haven't added any other extensions it can convert to, later when I add YAML I will have to use a match statement
    let content = std::fs::read_to_string(path)
        .better_expect("ERROR: Failed to read input file.", verbose)
        .trim_end()
        .to_string();

    let toml_des: TomlVal =
        toml::from_str(&content).better_expect("ERROR: Failed to deserialize file.", verbose);
    UniversalData::Structured(Vals::Toml(toml_des))
}
