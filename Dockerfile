FROM rust:1.74.1-bookworm

LABEL maintainer="yoshinorin"
WORKDIR /qualtet-mock

COPY ./ .

RUN apt-get update -y \
  && apt autoremove \
  && apt clean \
  && cargo build --release

ENTRYPOINT ["/qualtet-mock/target/release/qualtet-mock"]
