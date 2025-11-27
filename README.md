## Fiox

An ultra fast, minimal file conversion CLI tool written in **Rust**.
Supports **NDJSON**, **CSV**, **JSON**, and **TOML** formats.

---

### Features

- Convert between NDJSON, CSV, JSON, TOML and more as I add them!
- Validate files extremely quickly with optional detailed logs for debugging.
- Ultra-fast thanks to being written in highly optimized Rust.
- Simple yet professional command-line interface using **clap**.
- Clean, easy to understand repo structure and code.

---

### Installation

Clone the repository and build:

```bash
git clone https://github.com/tahamahmoud7097-wq/fiox.git
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
fiox convert <input_file> <output_file>

# validation
fiox validate <input_file>

# optional flag for extra debug logs
fiox validate <input_file> --verbose
fiox convert <input_file> <output_file> -v
```

**Note:** Flag and arguments ordering doesn't matter and the verbose flag works in any subcommands in short `-v`, and the long version `--verbose`.

---

### Plans

- [x] Modularize readers and writers.
- [x] Add clap for a better CLI and subcommand / flag support.
- [x] Add a flag for better debugging.
- [x] Add a validation subcommand.
- [ ] Implement streaming for all commands (except anything involving reading TOML) (in progress).
- [ ] Add support for more formats (in progress).
- [ ] Add parallelization for processing and serialization using rayon and a separate manual writer thread.
- [ ] Publish as a binary crate on **crates.io**.
- [ ] **Long-term goal:** Make a faster serializer than serde to use for the project and publish as a crate (not very realistic for now).

---

The project is always open for contributions from anybody, even if it's a single extra comment.
This project is licensed under the **MIT** license.
