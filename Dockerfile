# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:latest as cargo-build

# RUN apt-get update

# RUN apt-get install musl-tools -y

# RUN rustup target add x86_64-unknown-linux-musl

# WORKDIR /usr/src/temp_collector

# COPY Cargo.toml Cargo.toml

# RUN mkdir src/

# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# RUN rm -f target/x86_64-unknown-linux-musl/release/deps/temp_collector*

# COPY . .

# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

ADD --chown=rust:rust . ./

CMD ["ls -al"]

# Build our application.
RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 myapp

RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp

WORKDIR /home/myapp/bin/

COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/temp_collector .

RUN chown myapp:myapp temp_collector

USER myapp

CMD ["./temp_collector"]