<h1 align="center">tdlib-rust</h1>

[![Latest version](https://img.shields.io/crates/v/tdlib_rust.svg)](https://crates.io/crates/tdlib_rust)
[![Documentation](https://docs.rs/tdlib-rust/badge.svg)](https://docs.rs/tdlib-rust/latest/tdlib_rust/)
[![CI Linux](https://github.com/ankddev/tdlib-rust/actions/workflows/ci-linux.yml/badge.svg)](https://github.com/ankddev/tdlib-rust/actions/workflows/ci-linux.yml)
[![CI Windows](https://github.com/ankddev/tdlib-rust/actions/workflows/ci-windows.yml/badge.svg)](https://github.com/ankddev/tdlib-rust/actions/workflows/ci-windows.yml)
[![CI macOS](https://github.com/ankddev/tdlib-rust/actions/workflows/ci-macos.yml/badge.svg)](https://github.com/ankddev/tdlib-rust/actions/workflows/ci-macos.yml)
[![downloads](https://img.shields.io/crates/d/tdlib_rust)](https://crates.io/crates/tdlib_rust)
![license](https://img.shields.io/crates/l/tdlib_rust)

A Rust wrapper around the Telegram Database library. It includes a generator to automatically generate the types and functions from the TDLib's [Type Language](https://core.telegram.org/mtproto/TL) file.

## Why this fork?

This is an improved version of the [tdlib-rs](https://github.com/FedericoBruzzone/tdlib-rs) and [tdlib-rs](https://github.com/paper-plane-developers/tdlib-rs) libraries, with the following additional features:


1. It is cross-platform, it works on Windows, Linux and MacOS.
2. Not required `tdlib` to be compiled and installed on the system.
3. Not required `pkg-config` to build the library and associated exported variables.
2. Three different ways to build the library:
    - `download-tdlib`: download the precompiled library from the GitHub releases.
    - `local-tdlib`: use the `tdlib` installed on the system.
    - `pkg-config`: use the `pkg-config` to build the library.
5. It is possible to download the `tdlib` library from the GitHub releases.
6. Often updates to the latest TDLib version.
7. Updated structure of library.

## Information

We provide a precompiled version of the library for the supported platforms:

- Linux (x86_64)
- Windows (x86_64)
- macOS Intel (x86_64)
- macOS Apple Silicon (arm64)

We compile it in the CI and we upload the artifacts to the GitHub releases, so we can download it and use to build this library.

It's mainly created for using it in the currently in development oxiPlane client, but it should work also for any other Rust project.

Current supported TDLib version: [1.8.42](https://github.com/tdlib/td/commit/ef580cd3dd0e5223c2be503342dc29e128be866e).

## Cargo features

Please see the documentation of the module `build` for more information about the features [here](https://docs.rs/tdlib-rust/latest/tdlib_rust/build/index.html).
It functions that you can use to build the library in different ways.

### download-tdlib

If you don't want to compile and install the `tdlib` on your system manually, you should enable the `download-tdlib` feature in the `Cargo.toml` file:

```toml
[dependencies]
tdlib = { version = "...", features = [ "download-tdlib" ] }

[build-dependencies]
tdlib = { version = "...", features = [ "download-tdlib" ] }
```

```rust
// build.rs
fn main() {
    tdlib_rust::build::build(None);
}
```

### local-tdlib

`local-tdlib` require you to have the `tdlib` (version 1.8.42) compiled and installed on your system, and the following variables exported, for example in the `.bashrc` file:

```sh
# The path to the tdlib folder
export LOCAL_TDLIB_PATH=$HOME/lib/tdlib
```

Then you can enable the `local-tdlib` feature in the `Cargo.toml` file:

```toml
[dependencies]
tdlib = { version = "...", features = [ "local-tdlib" ] }

[build-dependencies]
tdlib = { version = "...", features = [ "local-tdlib" ] }
```

```rust
// build.rs
fn main() {
    tdlib_rust::build::build(None);
}
```

### pkg-config

If you want to use the `pkg-config` to build this library, you should enable the `pkg-config` feature in the `Cargo.toml` file:

```toml
[dependencies]
tdlib = { version = "...", features = [ "pkg-config" ] }

[build-dependencies]
tdlib = { version = "...", features = [ "pkg-config" ] }
```

```rust
// build.rs
fn main() {
    tdlib_rust::build::build(None);
}
```

remember to have the `tdlib` (version 1.8.42) compiled on your system, and the following variables exported, for example in the `.bashrc` file:

```sh
# pkg-config configuration
export PKG_CONFIG_PATH=$HOME/lib/tdlib/lib/pkgconfig/:$PKG_CONFIG_PATH

# dynmic linker configuration
export LD_LIBRARY_PATH=$HOME/lib/tdlib/lib/:$LD_LIBRARY_PATH
```

### docs

This feature skip the linking of the library and only generate the code of `generated.rs`.
Is used only for testing.

### bots-only-api

This feature enable the generation of the functions only used by Telegram bots.

## License

This repository are licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE][github-license-apache] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT][github-license-mit] or <http://opensource.org/licenses/MIT>)

at your option.

Please review the license file provided in the repository for more information regarding the terms and conditions of the license.

## Mantainers
  - [ANKDDEV](https://github.com/ankddev)

## Credits

- [grammers](https://github.com/Lonami/grammers): the `tdlib-tl-gen` and `tdlib-tl-parser` projects are forks of the `grammers-tl-gen` and `grammers-tl-parser` projects.
- [rust-tdlib](https://github.com/aCLr/rust-tdlib): for inspiration about some client code.
- [tdlib-rs](https://github.com/paper-plane-developers/tdlib-rs): for inspiration about the generator code.
- [tdlib-rs](https://github.com/FedericoBruzzone/tdlib-rs): initial version of the library.
