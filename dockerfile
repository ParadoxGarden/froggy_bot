FROM rust:1.69

WORKDIR /usr/src/froggy

COPY . .

RUN cargo install --path .

CMD ["froggy_bot"]