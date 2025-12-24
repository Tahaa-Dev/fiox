## fiox

The **fastest** multi-format file handling CLI tool, built in **Rust**.
Supports **NDJSON**, **CSV**, **JSON**, **TOML**, **TSV**, and **PSV** formats.

---

### Features

- Convert between NDJSON, CSV, JSON, TOML, TSV, PSV and more as I add them!
- Validate files extremely quickly with optional detailed logs for debugging.
- Ultra-fast thanks to being written in highly optimized Rust.
- Extremely memory efficient for low resource environments thanks to streaming architecture and optimized allocations.
- Intuitive and easy to use out of the box, built in 100% pure Rust which makes it easy to install and use.

---

### Installation

Clone the repository and build:

```bash
git clone https://github.com/Tahaa-Dev/fiox.git fiox

cd fiox

cargo build --release
# optional: move binary to ~/.local/bin for convenience
mv ~/fiox/target/release/fiox ~/.local/bin/fiox
```

**Note:** This installation is temporary until I publish as a binary crate on **crates.io**.

---

### Usage

```bash
# conversion
fiox convert <INPUT> -o <OUTPUT>

# validation
fiox validate <INPUT>

# options (flags)
fiox validate <INPUT> --verbose
fiox convert <INPUT> --output <OUTPUT> -a
```


---

### Options (flags)

| **Option (flag)**       | **Functionality**                                                                                                                                                                                                                                                              |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-v`, `--verbose`       | Enables extra debug logs in error messages.                                                                                                                                                                                                                                    |
| `-a`, `--append`        | fiox overwrites existing data in files by default, this flag makes it append into the output file instead. **WARNING**: This flag can lead to unexpected output on some formats like JSON, but I will add some guardrails to it, it is unstable and can break as of right now. |
| `-p`, `--parse-numbers` | This flag only affects CSV -> non-tabular format conversions only. It makes it so that fiox infers types and doesn't quote (stringify) numbers.                                                                                                                                |
| `-o`, `--output`        | Not an option, but is a flag you use to indicate which file is the output file.                                                                                                                                                                                                |

---

### Benchmarks

**Notes:** 
- All of these benchmarks were done using the same file but converted to other formats using fiox which is a 100k rows with 6 fields per row CSV file with 3 text columns, 2 number columns and a column of dates.
- All of these benchmarks were done using **hyperfine** on entry level hardware. (Ryzen 3 3100, 8GB of DDR3 RAM and a SATA SSD).

| **Benchmark**  | **fiox** | **Node.js** | **Miller / jq (C)**          |
| -------------- | -------- | ----------- | ---------------------------- |
| CSV to JSON    | ~112ms   | ~1.29s      | Miller: ~603ms               |
| CSV to TOML    | ~121ms   | ~1.60s      | No native TOML support       |
| JSON to TOML   | ~750ms   | ~7.2s       | No native TOML support       |
| TOML to JSON   | ~772ms   | ~9s         | No native TOML support       |
| CSV to NDJSON  | ~110ms   | ~1.2s       | Miller: ~2.85s              |
| JSON to NDJSON | ~600ms   | ~6s         | jq: ~2.77s \| Miller: ~2.92s |
| TOML to NDJSON | ~810ms   | ~8s         | No native TOML support       |
| NDJSON to JSON | ~272ms   | ~6.2s       | jq: ~2.65s \| Miller: ~2.88s  |
| NDJSON to TOML | ~300ms   | ~6.8s       | No native TOML support       |

As you can see from these benchmarks, **fiox is much faster than industry-standard file conversion tools**, fiox scales even better on better / server hardware (using SSH)!

---

### Plans

- [x] Modularize readers and writers.
- [x] Add clap for help and subcommand / flag support.
- [x] Add a flag for better debugging.
- [x] Add a validation subcommand.
- [x] Implement streaming for all commands (except TOML).
- [ ] Add support for more formats (in progress).
- [ ] Add parallelization for processing using rayon.
- [ ] Add more subcommands for more flexibility.
- [ ] Publish as a binary crate on **crates.io**.

---

### Notes

- fiox is licensed under the **MIT** license.
- For specifics about contributing to fiox, see **CONTRIBUTING.md**.
- fiox is still unstable and in heavy development, it's recommended that you only use fiox after v0.5.0 release.
