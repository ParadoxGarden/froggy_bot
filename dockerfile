FROM rust:1.69

WORKDIR /usr/src/froggy

COPY . .

WORKDIR /usr/src/froggy/Bot

RUN cargo install --path .

CMD ["froggy_bot"]