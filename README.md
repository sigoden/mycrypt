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

## Usage

```sh
# encrypt 
mycrypt encrypt -f secret-file.txt secret-file.bin
# encrypt with interactive editor
mycrypt encrypt secret-file.bin
# encrypt alias
mycrypt e secret-file.bin
# encrypt with pass from enviroment variable
MYCRYPT_PASS=123456 mycrypt encrypt secret-file.bin

# decrypt
mycrypt decrypt -o secret-file.txt secret-file.bin
# decrypt to stdout
mycrypt decrypt secret-file.bin > secret-file.txt
# decrypt alias
mycrypt d secret-file.bin
# decrypt with pass from enviroment variable
MYCRYPT_PASS=123456 decrypt secret-file.bin
```

## Contributing

Contributions are highly appreciated! Feel free to open issues or send pull requests directly.

# License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.