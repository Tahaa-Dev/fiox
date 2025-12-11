use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::utils::{BetterExpect, ByteTypes, WriterStreams, into_raw_bytes};

pub fn toml_writer(
    data_stream: WriterStreams<impl Iterator<Item = ByteTypes>>,
    path: &PathBuf,
    verbose: bool,
) {
    let file = OpenOptions::new().write(true).open(path).better_expect(
        format!(
            "ERROR: Failed to open output file [{}] for writing.",
            path.to_str().unwrap_or("[output.toml]")
        )
        .as_str(),
        verbose,
    );

    let mut buffered_writer = BufWriter::new(file);

    match data_stream {
        WriterStreams::LineByLine { iter } => {
            iter.for_each(|rec| {
                let toml_object = toml::from_slice::<toml::Value>(
                    into_raw_bytes(rec)
                    .as_slice())
                    .better_expect("ERROR: Failed to serialize value into TOML", verbose);

                buffered_writer.write(
                    toml::to_string(&toml_object)
                    .better_expect("INTERNAL ERROR: Failed to turn TOML into bytes for writing (possible OOM or deeply nested data)!", true)
                    .as_bytes())
                    .better_expect(
                    format!("ERROR: Failed to write TOML into output file [{}].", 
                            path
                                .to_str()
                                .unwrap_or("[output.toml]"))
                            .as_str(), 
                         verbose
                    );
            });
        }
        WriterStreams::Table { headers, iter } => {}
    }
}
