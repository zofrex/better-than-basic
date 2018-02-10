FROM rust:1.23.0 AS build

# Create dummy project to cache dependencies
RUN USER=root cargo new --bin --vcs none rust
WORKDIR /rust
COPY Cargo.* ./
RUN cargo build --release

# Remove artifact to force rebuild when we copy in the real project (timestamps!)
RUN rm target/release/better-than-basic

# Now compile actual project
COPY src src
RUN cargo build --release

# Slim down the build
FROM debian:jessie-slim

COPY --from=build /rust/target/release/better-than-basic /better-than-basic

COPY static /usr/share/better-than-basic/static
COPY templates /usr/share/better-than-basic/templates
COPY config-mine.toml /etc/better-than-basic/config.toml
COPY users.toml /etc/better-than-basic/users.toml

EXPOSE 3000

CMD /better-than-basic
