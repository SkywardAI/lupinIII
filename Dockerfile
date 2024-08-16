FROM rust:1.80.0-slim-bookworm

WORKDIR app
COPY . .
RUN apt update && apt install -y protobuf-compiler libssl-dev
RUN cargo build --release
RUN rm src/*.rs

FROM debian:bookworm-slim
ARG APP=lupinIII
WORKDIR app

COPY --from=0 /app/target/release/$APP .

ENTRYPOINT ["./lupinIII"]
