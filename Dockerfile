FROM rust:1.79.0-slim-bookworm

WORKDIR app
COPY . .
RUN cargo build --release
RUN rm src/*.rs

FROM debian:bookworm-slim
ARG APP=lupinIII
WORKDIR app

COPY --from=0 /app/target/release/$APP .

ENTRYPOINT ["./lupinIII"]
