use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde_json::Value;

use crate::utils::{BetterExpect, DataTypes, WriterStreams, into_byte_record};

#[inline]
pub fn ndjson_writer(
    data_stream: WriterStreams<impl Iterator<Item = DataTypes>>,
    verbose: bool,
    file: File,
    parse_numbers: bool,
) {
    let mut writer = BufWriter::new(file);

    match data_stream {
        WriterStreams::Values { iter } | WriterStreams::Ndjson { values: iter } => {
            iter.for_each(|item| {
                let json =
                    serde_json::to_value(item).better_expect("ERROR: Invalid input file.", verbose);

                if let Value::Array(arr) = json {
                    arr.iter().enumerate().for_each(|(idx, obj)| {
                        serde_json::to_writer(&mut writer, obj).better_expect(
                            format!("ERROR: Failed to write object at line [{}].", idx + 1)
                                .as_str(),
                            verbose,
                        );

                        writeln!(writer).better_expect(
                            format!(
                                "ERROR: Failed to write newline delimiter at line [{}].",
                                idx + 1
                            )
                            .as_str(),
                            verbose,
                        );
                    });
                }
            });
        }

        WriterStreams::Table { headers, iter } => {
            let mut esc_buf: Vec<u8> = Vec::with_capacity(10);

            let headers: Vec<String> = headers
                .iter()
                .map(|h| {
                    h.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\t', "\\t")
                        .replace('\r', "\\r")
                        .replace('\n', "\\n")
                })
                .collect();

            iter.for_each(|rec| {
                writer.write(b"{").better_expect(
                    "ERROR: Failed to write opening bracket for object into output file.",
                    verbose,
                );

                let mut first_value = true;

                let record = into_byte_record(rec);
                headers.iter().zip(record.iter()).for_each(|(h, v)| {
                    esc_buf.clear();
                    if matches!(v, b"true" | b"false" | b"null")
                        || (parse_numbers
                            && v.first()
                                .is_some_and(|b| *b == b'-' || *b == b'+' || b.is_ascii_digit())
                            && v.last().is_some_and(|b| b.is_ascii_digit())
                            && std::str::from_utf8(v).unwrap_or("").parse::<f64>().is_ok())
                    {
                        esc_buf.extend_from_slice(v);
                    } else {
                        esc_buf.push(b'"');
                        let mut start = 0usize;
                        v.iter().enumerate().for_each(|(idx, byte)| {
                            let chunk = &v[start..idx];
                            match *byte {
                                b'\\' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\\\");
                                    start = idx + 1;
                                }
                                b'\n' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\n");
                                    start = idx + 1;
                                }
                                b'"' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\\"");
                                    start = idx + 1;
                                }
                                b'\t' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\t");
                                    start = idx + 1;
                                }
                                b'\r' => {
                                    esc_buf.extend_from_slice(chunk);
                                    esc_buf.extend_from_slice(b"\\r");
                                    start = idx + 1;
                                }
                                _ => {}
                            }
                        });
                        if start < v.len() {
                            esc_buf.extend_from_slice(&v[start..]);
                        }
                        esc_buf.push(b'"');
                    } 
                    if first_value {
                        writer.write_all(b"\"").better_expect("ERROR: Failed to write opening quote for key in key-value pair.", verbose);

                        writer.write_all(h.as_bytes()).better_expect(
                            "ERROR: Failed to write key in key-value pair into output file.",
                            verbose,
                        );

                        first_value = false;
                    } else {
                        writer.write_all(b", ").better_expect(
                            "ERROR: Failed to write comma into output file.",
                            verbose,
                        );

                        writer.write_all(b"\"").better_expect("ERROR: Failed to write opening quote for key in key-value pair.", verbose);

                        writer.write_all(h.as_bytes()).better_expect(
                            "ERROR: Failed to write key in key-value pair into output file.",
                            verbose,
                        );
                    }
                    writer.write_all(b"\"").better_expect("ERROR: Failed to write closing quote for key in key-value pair.", verbose);

                    writer.write_all(b": ").better_expect("ERROR: Failed to write colon between key and value in key-value pair into output file.", verbose);

                    writer.write_all(esc_buf.as_slice()).better_expect("ERROR: Failed to write value for key-value pair into output file.", verbose);
                });

                writer.write_all(b"}\n").better_expect("ERROR: Failed to write closing bracket and newline delimiter for object into output file.", verbose);
            });

            writer
                .flush()
                .better_expect("ERROR: Failed to flush final bytes into output file.", verbose);
        }

        _ => unreachable!(),
    }
}
