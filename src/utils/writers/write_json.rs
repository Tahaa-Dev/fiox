use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use crate::utils::{BetterExpect, ByteTypes, WriterStreams, into_raw_bytes};

pub fn write_json(
    data_stream: WriterStreams<impl Iterator<Item = ByteTypes>>,
    path: &PathBuf,
    verbose: bool,
) {
    let file = File::open(path).better_expect(
        format!(
            "ERROR: Couldn't open output file [{}] for writing.",
            path.to_str().unwrap_or("[output.json]")
        )
        .as_str(),
        verbose,
    );

    let mut buffered_writer = BufWriter::new(&file);

    match data_stream {
        WriterStreams::LineByLine { iter } => {
            iter.for_each(|obj| {
                // get raw bytes from the `ByteTypes` enum, very cheap
                let object = &into_raw_bytes(obj);

                serde_json::to_writer_pretty(&mut buffered_writer, object).better_expect(
                    format!(
                        "ERROR: Failed to write JSON into output file [{}].",
                        path.to_str().unwrap_or("[output.json]")
                    )
                    .as_str(),
                    verbose,
                );

                writeln!(buffered_writer).better_expect(
                    "ERROR: Failed to write a newline into the output file.",
                    verbose,
                );
            });

            buffered_writer
                .flush()
                .better_expect("ERROR: Failed to flush writer into output file.", verbose);
        }

        WriterStreams::Table { headers, iter } => {
            println!("Hello, World!");
        }
    }
}
