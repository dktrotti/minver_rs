![CI Status](https://github.com/dktrotti/minver_rs/actions/workflows/rust.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/minver_rs.svg)](https://crates.io/crates/minver_rs)
# minver_rs

Implementation of https://github.com/adamralph/minver using Rust

## How to use

### CLI
The CLI binary allows minver to be run as a regular command.
1. Install `minver_rs` using `cargo`
```
> cargo install minver_rs
```
2. Run `minver` (Note: Make sure that `.cargo\bin` is on your `PATH`)
```
> minver
1.2.3
```

### Build Util
The build util binary is a tool that can be integrated into your build to automatically update `Cargo.toml` with the correct version.
1. Install `minver_rs` using `cargo`
```
> cargo install minver_rs
```
2. Set the environment variable `MINVER_UPDATE_VERSION`
```
> export MINVER_UPDATE_VERSION=1
```
3. Run `minver_build_util` (Note: Make sure that `.cargo\bin` is on your `PATH`)
```
> minver_build_util
```

### As a build dependency
`minver_rs` can also be used directly in `build.rs`.
1. Add a build dependency on `minver_rs`
```
[build-dependencies]
minver_rs = "x.y.z"
```
2. (Optional) Update `version` in `Cargo.toml` to be `0.0.0`. While this is not strictly necessary, it helps to make it apparent that the version is handled automatically by minver.
3. Add a file called `build.rs` to your project root.
```
use minver_rs::build_utils;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_utils::default_build_action();
}
```
4. Set the environment variable `MINVER_UPDATE_VERSION`
```
> export MINVER_UPDATE_VERSION=1
```
5. Build your crate
```
> cargo build
```