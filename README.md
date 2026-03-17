# Hydrogen

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

A fast, lightweight file compression CLI tool built in Rust using Huffman coding.

## How it works

Hydrogen reads a file as raw bytes, builds a Huffman tree based on byte frequency, and encodes the data using variable-length prefix-free codes — shorter codes for frequent bytes, longer codes for rare ones. The compressed output is stored in a custom `.h2` binary format that includes the frequency table for decompression.

## Installation

```sh
git clone https://github.com/yourusername/hydrogen.git
cd hydrogen
cargo build --release
```

The binary will be at `target/release/hydrogen` (or `hydrogen.exe` on Windows).

## Usage

**Compress a file:**

```sh
hydrogen compress myfile.txt                     # outputs myfile.h2
hydrogen compress myfile.txt -o backup           # outputs backup.h2
```

**Decompress a file:**

```sh
hydrogen decompress myfile.h2                    # outputs myfile.txt
hydrogen decompress myfile.h2 -o original.md     # outputs original.md
```

Compressed files always use the `.h2` extension. On decompression, the default output is `.txt` — use `-o` to specify a different name or extension.

**Help:**

```sh
hydrogen --help
hydrogen compress --help
```

## .h2 File Format

| Section            | Size                  | Description                          |
| ------------------ | --------------------- | ------------------------------------ |
| Padding            | 1 byte                | Number of padding bits in last byte  |
| Unique byte count  | 1 byte                | Number of unique bytes minus 1       |
| Frequency table    | 5 bytes × unique count| 1 byte character + 4 bytes frequency |
| Compressed data    | Variable              | Huffman-encoded bit stream           |

## Running Tests

```sh
cargo test
```

Tests cover roundtrip compression/decompression for edge cases (empty input, single byte, all 256 byte values) as well as full CLI integration tests.