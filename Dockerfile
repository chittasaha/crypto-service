FROM ekidd/rust-musl-builder:stable as build


WORKDIR /app

# copy over your manifests
COPY ./src ./src
COPY ./Cargo.toml ./
# this build step will cache your dependencies
RUN cargo build --release 


# our final base
FROM alpine:latest
EXPOSE 8080
# copy the build artifact from the build stage
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/crypto-service .

CMD ["./crypto-service"]
