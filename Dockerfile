FROM rust:1.73.0 AS chef

WORKDIR /app
RUN cargo install cargo-chef
RUN apt update && apt install lld clang -y

FROM chef as planner

COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin rest-api-server

FROM debian:bookworm-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rest-api-server rest-api-server
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./rest-api-server"]
