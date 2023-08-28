FROM rust:1.68-alpine as build

RUN apk update
RUN apk add libpq-dev

RUN apk add openssl-dev musl-dev
RUN apk add pkgconfig

RUN USER=root cargo new --bin server
WORKDIR /server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/server*
RUN cargo build --release

FROM alpine:3.16.0 as runtime
COPY --from=build /server/target/release/server .
COPY --from=build /server/target/release/seed .

CMD ["./server"]
