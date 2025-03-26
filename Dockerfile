# Build image
FROM rust:1.85.1-bookworm as builder

LABEL maintainer="yoshinorin"
WORKDIR /qualtet-mock

COPY ./ .

RUN apt-get update -y \
  && apt autoremove \
  && apt clean \
  && cargo build --release

# Runtime image
FROM --platform=linux/x86_64 gcr.io/distroless/cc-debian12 AS base
WORKDIR /qualtet-mock

LABEL maintainer="yoshinorin"

COPY --from=builder /qualtet-mock/target/release/qualtet-mock .
COPY --from=builder /qualtet-mock/src/resources ./src/resources

EXPOSE 9002

ENTRYPOINT ["./qualtet-mock"]
