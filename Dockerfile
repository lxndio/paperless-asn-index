FROM rust:latest as build

RUN USER=root cargo new --bin paperless-asn-index
WORKDIR /paperless-asn-index

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/paperless_asn_index*
RUN cargo build --release

FROM rust:latest

COPY --from=build /paperless-asn-index/target/release/paperless-asn-index .
COPY ./static ./static

CMD ["./paperless-asn-index"]
