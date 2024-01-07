FROM rust AS build

RUN USER=root cargo new --bin photon-api
WORKDIR /photon-api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && rm src/*.rs

COPY ./src ./src

RUN rm target/release/deps/photon_api* && cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
RUN update-ca-certificates

COPY --from=build /photon-api/target/release/photon-api .

EXPOSE 2322

CMD ["./photon-api"]