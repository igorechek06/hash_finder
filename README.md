# Hash Finder

A command-line tool to find SHA256 hashes with a specified number of trailing zeros, written in Rust.

## Build

1.  **[Install Rust](https://www.rust-lang.org/tools/install)**
2.  **Clone**

```sh
git clone https://github.com/igorechek06/hash_finder.git
cd hash_finder
```

3.  **Build**

```sh
cargo build --release
```

## Usage

The executable is located at `./target/release/hash_finder`.

### Command-Line Options:

- `-N <ZEROS>`: Number of trailing zeros (Default: `3`, Range: 1-64).
- `-F <RESULTS>`: Number of hashes to find (Default: `1`, Greater then 1).
- `-h, --help`: Display help information.

### Example

To find 5 hashes with 4 trailing zeros:

```sh
./target/release/hash_finder -N 4 -F 5
```

Alternatively, you can use `cargo run`:

```sh
cargo run --release -- -N 4 -F 5
```
