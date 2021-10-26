# mycrypt

Encrypt/decrypt your file with aes256


## Installation

### Cargo

You can also install the latest version (or a specific commit) of mycrypt directly from GitHub.

```shell
cargo install --git https://github.com/sigoden/mycrypt.git
```

### Prebuilt binaries

Archives of prebuilt binaries are available on [GitHub Release][gh-release] for Linux, maxOS and Windows. Download a compatible binary for your system. For convenience, make sure you place mycrypt under $PATH if you want access it from the command line.

[gh-release]: https://github.com/sigoden/mycrypt/releases

### Build from source

sfz is written in Rust. You need to [install Rust][install-rust] in order to compile it.

```shell
$ git clone https://github.com/sigoden/mycrypt.git
$ cd sfz
$ cargo build --release
$ ./target/release/mycrypt
```

[install-rust]: https://www.rust-lang.org/install.html

## Usage

```sh
# encrypt 
mycrypt encrypt -f secret-file.txt secret-file.bin
mycrypt encrypt secret-file.bin

# decrypt
mycrypt decrypt -o secret-file.txt secret-file.bin
mycrypt decrypt secret-file.bin > secret-file.txt
```

## Contributing

Contributions are highly appreciated! Feel free to open issues or send pull requests directly.

# License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.