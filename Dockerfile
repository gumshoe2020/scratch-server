# Stage 1: build. Has the Rust compiler, a shell, apk: everything needed to build.
FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# On Alpine the default target is musl, so the binary is fully static.
RUN cargo build --release --locked

# Stage 2: run. Starts from nothing; only the binary is copied in.
# No compiler, no shell, no interpreter, no package manager, no libc.
FROM scratch
COPY --from=build /src/target/release/scratch-server /scratch-server
USER 65532:65532
EXPOSE 8080
HEALTHCHECK --interval=10s --timeout=3s CMD ["/scratch-server", "--healthcheck"]
ENTRYPOINT ["/scratch-server"]
