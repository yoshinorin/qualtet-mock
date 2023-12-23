# Qualtet mock

[![CI](https://github.com/yoshinorin/qualtet-mock/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/yoshinorin/qualtet-mock/actions/workflows/ci.yml)

This project target is create a mock server of [Qualtet](https://github.com/yoshinorin/qualtet). It will be used for [Quintet](https://github.com/yoshinorin/quintet)'s E2E test.

## Requirements

* rustup 1.26.0
* rustc 1.74.1
* cargo-watch (Optional)
* clippy (Optional: for lint)

## Preconditions

Install [cargo-watch](https://github.com/watchexec/cargo-watch).

```
$ cargo install cargo-watch@8.4.1
```

## API Docs

Please see [Qualtet's REST API docs](https://yoshinorin.github.io/qualtet/rest-api/) instead.

## Supported Endpoint

- [x] `/archives`
- [x] `/articles`
  - [ ] `/articles?{queryParams}`
- [x] `/contents/standard`: standard post.
  - [x] `/contents/empty-robots`: robotesAttrobutes are empty post.
  - [x] `/contents/empty-tags`: tags are empty post.
  - [x] `/contents/partially-robots`: with partially robotesAttrobutes post.
  - [x] `/contents/with-externalresources`: with externalResources post.
  - [x] `/contents/without-robots`: without robotesAttrobutes post.
  - [x] `/contents/without-tags`: without tags post.
- [ ] `/feeds`
- [ ] `/search`
- [x] `/series`
  - [ ] `/series/{seriesName}`
- [ ] `/sitemaps`
- [x] `/system/health`
- [x] `/system/metadata`
- [x] `/tags`
  - [x] `/tags/{tagName}`
  - [ ] `/tags/{tagName}?{queryParams}`

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

## Docker

How to build.

```sh
// with cache
$ docker build . --progress=plain
// without cache
$ docker build . --progress=plain --no-cache
```

## LICENSE

This code is open source software licensed under the [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0.html).
