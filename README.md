# uuid-v4-cli [![crates.io version](https://img.shields.io/crates/v/uuid-v4-cli.svg)](https://crates.io/crates/uuid-v4-cli) [![crates.io downloads](https://img.shields.io/crates/d/uuid-v4-cli.svg)](https://crates.io/crates/uuid-v4-cli)

A CLI tool to generate UUID V4 which supports both native and WebAssembly

*Note: This project is a fork of the original Rust implementation: [uuid-rs](https://github.com/uuid-rs/uuid).*

## Installation

You can install this using the `cargo install` command:

```bash
$ cargo install uuid-v4-cli
```

### WebAssembly

This application also provides a wasm package.
You can install it using [`wapm`](https://wapm.io/help/install) by the following command:

```bash
$ wapm install ken-matsui/uuid
```

## Usage

```bash
$ uuid --help
uuid-v4-cli 0.3.0
A CLI tool to generate UUID V4

USAGE:
    uuid [OPTIONS]

OPTIONS:
    -H, --hyphenated    Show with hyphens
        --help          Print help information
    -u, --uppercase     Show as uppercase (default: lowercase)
        --urn           Show as a urn
    -V, --version       Print version information
```

### WebAssembly

```bash
$ wapm run uuid -- --help
uuid-v4-cli 0.3.0
A CLI tool to generate UUID V4

USAGE:
    uuid [OPTIONS]

OPTIONS:
    -H, --hyphenated    Show with hyphens
        --help          Print help information
    -u, --uppercase     Show as uppercase (default: lowercase)
        --urn           Show as a urn
    -V, --version       Print version information
```

## Examples

### Simple UUID

```bash
$ uuid
4611494855814da2a559fd0d6d422766
```

#### with uppercase

```bash
$ uuid -u
4D41163F06F7404BBB3A6C357062DAE0
```

### Hyphenated UUID

```bash
$ uuid -H
b6dc2c9c-6408-433f-8e5b-f91677cad729
```

#### with uppercase

```bash
$ uuid -H -u
2FBC671A-3BD9-4BDC-B5B2-95C18CBEF900
```

### Urn UUID

```bash
$ uuid --urn
urn:uuid:9756279b-886a-4bc7-83ae-cefe69e8397e
```

#### with uppercase

```bash
$ uuid --urn -u
urn:uuid:5B91ECE3-A85D-481E-BA45-69AC04ECC0F0
```

### WebAssembly

### Simple UUID

```bash
$ wapm run uuid
4611494855814da2a559fd0d6d422766
```

#### with uppercase

```bash
$ wapm run uuid -- -u
4D41163F06F7404BBB3A6C357062DAE0
```

### Hyphenated UUID

```bash
$ wapm run uuid -- -H
b6dc2c9c-6408-433f-8e5b-f91677cad729
```

#### with uppercase

```bash
$ wapm run uuid -- -H -u
2FBC671A-3BD9-4BDC-B5B2-95C18CBEF900
```

### Urn UUID

```bash
$ wapm run uuid -- --urn
urn:uuid:9756279b-886a-4bc7-83ae-cefe69e8397e
```

#### with uppercase

```bash
$ wapm run uuid -- --urn -u
urn:uuid:5B91ECE3-A85D-481E-BA45-69AC04ECC0F0
```

## Contribution

### Build

```bash
$ cargo build
```

Or you can directly execute the binary:

```bash
$ cargo run
```

#### WebAssembly

```bash
$ rustup target add wasm32-wasi
$ cargo build --target wasm32-wasi
$ wasmer run target/wasm32-wasi/debug/uuid.wasm
```

### Test

This command can also test C API.

```bash
$ cargo build
$ cargo test
```

### Publish

#### [GitHub Releases](https://github.com/ken-matsui/uuid-v4-cli/tags)

```bash
$ git tag v0.1.0
$ git push origin v0.1.0
```

#### [crates.io](https://crates.io/)

```bash
$ cargo publish
```

#### [wapm.io](https://wapm.io/)

```bash
$ cargo build --release --target wasm32-wasi
$ wapm publish
```
