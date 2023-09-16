# Better support of Docker layer caching in Cargo:
# https://hackmd.io/@kobzol/S17NS71bh#Using-cargo-chef
# https://github.com/LukeMathWalker/cargo-chef#without-the-pre-built-image


# install cargo-chef and toolchain, to be reused in other stages
FROM rust:1.72-bookworm as chef
RUN cargo install cargo-chef
RUN rustup install stable # should match the channel in rust-toolchain.toml
WORKDIR app



# only prepares the build plan
FROM chef as planner
COPY Cargo.toml Cargo.lock rust-toolchain.toml .
COPY src src
# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json



# build the project with a cached dependency layer
FROM chef as builder
# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this layer is cached as long as `recipe.json` doesn't change.
COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
RUN cargo chef cook --release --recipe-path recipe.json
# Build the full project
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations
COPY ./static ./static
RUN SQLX_OFFLINE=true cargo build --locked --release --features embed_migrations



# copy the binary to a minimal image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install --yes ca-certificates openssl sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/y /usr/local/bin/app
CMD ["app"]
