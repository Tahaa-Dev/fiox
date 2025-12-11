use std::{fs::File, io::BufReader};

use crate::utils::{BetterExpect, ByteTypes, WriterStreams};

pub fn json_decoder(
    reader: serde_json::Deserializer<serde_json::de::IoRead<BufReader<File>>>,
    verbose: bool,
) -> WriterStreams<impl Iterator<Item = ByteTypes>> {
    let iter = reader.into_iter::<serde_json::Value>().map(move |obj| {
        // SAFETY: to_vec only fails on OOM, which is unrecoverable
        // I set verbose=true to show full error details since this indicates
        // a catastrophic failure, not user error
        ByteTypes::Raw(
            serde_json::to_vec(
                &obj.better_expect("ERROR: Invalid JSON values in input JSON file.", verbose),
            )
            .better_expect(
                "INTERNAL ERROR: Failed to serialize JSON into raw bytes (possible OOM or deeply nested data)!",
                true,
            ),
        )
    });

    WriterStreams::LineByLine { iter }
}
