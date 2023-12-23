FROM rust:1.74.1-bookworm

LABEL maintainer="yoshinorin"
WORKDIR /qualtet-mock

COPY ./ .

RUN apt-get update -y \
  && apt autoremove \
  && apt clean \
  && cargo build --release \
  && rm -rf ./target/release/build ./target/release/deps ./target/release/.fingerprint ./target/release/examples ./target/release/qualtet_mock.pdb ./target/release/qualtet_mock.exe ./target/release/qualtet_mock.d

ENTRYPOINT ["/qualtet-mock/target/release/qualtet-mock"]
