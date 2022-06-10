# seawater
[![Semantic Versioning 2.0.0](https://img.shields.io/badge/semver-2.0.0-standard.svg)](https://semver.org/)
[![Linux](https://svgshare.com/i/Zhy.svg)](https://svgshare.com/i/Zhy.svg)
[![Windows](https://svgshare.com/i/ZhY.svg)](https://svgshare.com/i/ZhY.svg)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![GitHub license](https://img.shields.io/github/license/ii887522/seawater.svg)](https://github.com/ii887522/seawater/blob/master/LICENSE)
[![Github tag](https://badgen.net/github/tag/ii887522/seawater)](https://github.com/ii887522/seawater/tags/)

A high performance entity component system useful as a backend for developing GUI applications and video games.

## Table of contents
- [Prerequisites](https://github.com/ii887522/seawater#prerequisites)
- [Format the project](https://github.com/ii887522/seawater#format-the-project)
- [Automatically format the project on change](https://github.com/ii887522/seawater#automatically-format-the-project-on-change)
- [Lint the project](https://github.com/ii887522/seawater#lint-the-project)
- [Automatically lint the project on change](https://github.com/ii887522/seawater#automatically-lint-the-project-on-change)
- [Build the project](https://github.com/ii887522/seawater#build-the-project)
- [Automatically build the project on change](https://github.com/ii887522/seawater#automatically-build-the-project-on-change)
- [Test the project](https://github.com/ii887522/seawater#test-the-project)
- [Automatically test the project on change](https://github.com/ii887522/seawater#automatically-test-the-project-on-change)

## Prerequisites
- Windows 11 or Linux
- [Visual Studio Code](https://code.visualstudio.com/) with plugins:
  - Better TOML
  - CodeLLDB
  - EditorConfig for VS Code
  - Markdown All in One
  - rust-analyzer
  - YAML
- [Rust 1.60.0](https://www.rust-lang.org/) and later
- [rustfmt 1.4.38](https://github.com/rust-lang/rustfmt) and later
- [clippy 0.1.60](https://github.com/rust-lang/rust-clippy) and later
- [cargo-watch 8.1.1](https://github.com/watchexec/cargo-watch) and later

## Format the project
```sh
cargo fmt
```

## Automatically format the project on change
```sh
cargo watch -x fmt
```

## Lint the project
```sh
cargo clippy --all-features
```

## Automatically lint the project on change
```sh
cargo watch -x "clippy --all-features"
```

## Build the project
```sh
cargo build
```

## Automatically build the project on change
```sh
cargo watch -x build
```

## Test the project
```sh
cargo test
```

## Automatically test the project on change
```sh
cargo watch -x test
```
