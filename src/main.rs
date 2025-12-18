// MiMalloc setup because MiMalloc is much faster than the default rust allocator
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod utils;
use clap::Parser;
use colored::Colorize;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::exit;
use utils::*;

fn main() {
    let args: FioxArgs = cli::FioxArgs::parse();

    match args.cmd {
        Commands::Convert { verbose, input, output, append, parse_numbers } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!("{}", "ERROR: Input file doesn't exist.".red().bold());
                exit(1);
            }

            let output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .open(&output)
                .better_expect("ERROR: Failed to open output file.", verbose);

            let input_ext = input
                .extension()
                .better_expect("ERROR: Input file has no valid extension.", verbose)
                .to_str()
                .better_expect("ERROR: Input file has no valid extension.", verbose);
            let output_ext = output
                .extension()
                .better_expect("ERROR: Output file has no valid extension.", verbose)
                .to_str()
                .better_expect("ERROR: Output file has no valid extension.", verbose);

            let now = std::time::Instant::now();

            let (json, toml, csv, ndjson, idx) = get_data_stream(input_ext, &input, verbose);

            match idx {
                0 => match_output(json, output_file, verbose, output_ext, parse_numbers),
                1 => match_output(toml, output_file, verbose, output_ext, parse_numbers),
                2 => match_output(csv, output_file, verbose, output_ext, parse_numbers),
                3 => match_output(ndjson, output_file, verbose, output_ext, parse_numbers),
                _ => unreachable!(),
            };

            println!(
                "Finished converting {} -> {} in {:?}",
                input.to_str().unwrap_or("input file").bright_green().bold(),
                output.to_str().unwrap_or("output file").bright_green().bold(),
                now.elapsed()
            );
        }

        Commands::Validate { input, verbose } => {
            // Check if input exists
            if !Path::new(&input).exists() {
                eprintln!("{}", "ERROR: Input file doesn't exist for validation.".red().bold());
                exit(1);
            }

            let input_ext = input
                .extension()
                .better_expect("ERROR: Input file has no valid extension.", verbose)
                .to_str()
                .better_expect("ERROR: Input file has no valid extension.", verbose);

            match input_ext {
                "json" => json_validator::validate_json(&input, verbose),
                "toml" => toml_validator::validate_toml(&input, verbose),
                "csv" => csv_validator::validate_csv(&input, verbose),
                "ndjson" => ndjson_validator::validate_ndjson(&input, verbose),
                _ => {
                    let repo_link =
                        "https://github.com/Tahaa-Dev/fiox".truecolor(16, 101, 230).bold();
                    eprintln!(
                        "{} \n Open an issue at {}",
                        format!(
                            "ERROR: Input extension \"{}\" is not supported currently.",
                            input_ext
                        )
                        .red()
                        .bold(),
                        repo_link
                    );
                    exit(1);
                }
            };
            println!(
                "{}",
                format!("Input file [{}] is valid!", input.to_str().unwrap_or("inputFile"))
                    .green()
                    .bold()
            );
        }
    }
}

fn get_data_stream(
    input_ext: &str,
    input: &PathBuf,
    verbose: bool,
) -> (
    WriterStreams<impl Iterator<Item = DataTypes>>,
    WriterStreams<impl Iterator<Item = DataTypes>>,
    WriterStreams<impl Iterator<Item = DataTypes>>,
    WriterStreams<impl Iterator<Item = DataTypes>>,
    i8,
) {
    let mut data1 = WriterStreams::Temp {};
    let mut data2 = WriterStreams::Temp {};
    let mut data3 = WriterStreams::Temp {};
    let mut data4 = WriterStreams::Temp {};
    let num;
    match input_ext {
        "json" => {
            data1 = json_decoder::json_decoder(json_reader::json_reader(input, verbose), verbose);
            num = 0;
        }
        "toml" => {
            data2 = toml_decoder::toml_decoder(toml_reader::toml_reader(input, verbose));
            num = 1;
        }
        "csv" => {
            data3 = csv_decoder::csv_decoder(csv_reader::csv_reader(input, verbose), verbose);
            num = 2
        }
        "ndjson" => {
            data4 = ndjson_decoder::ndjson_decoder(
                ndjson_reader::ndjson_reader(input, verbose),
                verbose,
            );
            num = 3
        }
        _ => {
            let repo_link = "https://github.com/Tahaa-Dev/fiox".truecolor(16, 101, 230).bold();
            eprintln!(
                "{} \n Open an issue at {}",
                format!("ERROR: Intput extension \"{}\" is not supported currently.", input_ext)
                    .red()
                    .bold(),
                repo_link
            );
            exit(1);
        }
    };
    (data1, data2, data3, data4, num)
}

fn match_output(
    data: WriterStreams<impl Iterator<Item = DataTypes>>,
    output: std::fs::File,
    verbose: bool,
    output_ext: &str,
    parse_numbers: bool,
) {
    match output_ext {
        "json" => write_json::write_json(data, output, verbose, parse_numbers),
        "toml" => toml_writer::toml_writer(data, output, verbose, parse_numbers),
        "csv" => csv_writer::csv_writer(data, output, verbose),
        "ndjson" => ndjson_writer::ndjson_writer(data, verbose, output, parse_numbers),
        _ => {
            let repo_link = "https://github.com/Tahaa-Dev/fiox".truecolor(16, 101, 230).bold();
            eprintln!(
                "{} \n Open an issue at {}",
                format!("ERROR: Output extension \"{}\" is not supported currently.", output_ext)
                    .red()
                    .bold(),
                repo_link
            );
            exit(1);
        }
    };
}
