FROM rust:1.49-slim as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.49-slim as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.49-slim as builder
WORKDIR /app
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime
WORKDIR /app
COPY --from=builder /app/target/release/bin .
# TODO: Remove copying environment variable - generally bad practice.
COPY .env .
CMD ["./bin"]
EXPOSE 80
