FROM rust:1.69

WORKDIR /usr/src/froggy

COPY . .

RUN cargo build

CMD ["./target/debug/froggy_bot"]