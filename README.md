# Qualtet mock

[![CI](https://github.com/yoshinorin/qualtet-mock/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/yoshinorin/qualtet-mock/actions/workflows/ci.yml)

This project target is create a mock server of [Qualtet](https://github.com/yoshinorin/qualtet). It will be used for [Quintet](https://github.com/yoshinorin/quintet)'s E2E test.

## Requirements

* rustup 1.27.1
* rustc 1.79.0
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

- [x] `/v1/archives`
- [x] `/v1/articles`
  - [ ] `/v1/articles?{queryParams}`
- [ ] `/v1/contents/articles/standard/`: Standard post (not nested).
- [x] `/v1/contents/articles/nested/standard/`: Standard post.
  - [x] `/v1/contents/articles/nested/empty-robots/`: robotesAttrobutes are empty post.
  - [x] `/v1/contents/articles/nested/empty-tags/`: tags are empty post.
  - [x] `/v1/contents/articles/nested/partially-robots/`: with partially robotesAttrobutes post.
  - [x] `/v1/contents/articles/nested/with-externalresources/`: With externalResources post.
  - [x] `/v1/contents/articles/nested/without-robots/`: without robotesAttrobutes post.
  - [x] `/v1/contents/articles/nested/without-tags/`: Without tags post.
- [x] `/v1/feeds/index`
- [ ] `/v1/search`
- [x] `/v1/series`
  - [ ] `/v1/series/{seriesName}`
- [x] `/v1/sitemaps/`
- [x] `/v1/system/health`
- [x] `/v1/system/metadata`
- [x] `/v1/tags`
  - [x] `/v1/tags/{tagName}`
  - [ ] `/v1/tags/{tagName}?{queryParams}`

## Docker Support

Qualtet-mock provides docker image. Please see [GitHub Container Repository](https://github.com/yoshinorin/qualtet-mock/pkgs/container/docker-qualtet-mock).

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
