FROM rust:latest AS builder
WORKDIR /usr/src/eden
COPY . .
RUN cargo clean && cargo build --release
CMD ["target/release/eden"]

# Clean up the big image by only including the compiled application.
# (I am having issues getting this working, so for now the container size will be large.)
# FROM debian:buster-slim
# COPY --from=builder /usr/src/eden/target/release/eden /usr/src/eden
# WORKDIR /usr/src/eden
# CMD ["./eden"]