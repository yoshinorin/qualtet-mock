# Qualtet mock

[![CI](https://github.com/yoshinorin/qualtet-mock/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/yoshinorin/qualtet-mock/actions/workflows/ci.yml)

This project target is create a mock server of [Qualtet](https://github.com/yoshinorin/qualtet). It will be used for [Quintet](https://github.com/yoshinorin/quintet)'s E2E test.

## Requirements

* rustup 1.26.0
* rustc 1.74.1
* cargo-watch (Optional)

## Preconditions

Install [cargo-watch](https://github.com/watchexec/cargo-watch).

```
$ cargo install cargo-watch@8.4.1
```

## Run local server

After running the command, a local server starts at `http://localhost:9002`.

```
// with auto-reload
$ cargo watch -w src -x run

// without auto-reload
$ cargo run
```

## Code format

```
$ cargo fmt
```
