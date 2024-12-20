FROM --platform=$BUILDPLATFORM public.ecr.aws/docker/library/rust:1.83.0-slim as builder

ARG APP_NAME=orde-rs-service
ARG RUST_MUSL_TARGET=aarch64-unknown-linux-musl

# Install musl-tools for static compilation and ca-certificates for the build
RUN apt-get update && \
    apt-get install -y musl-tools ca-certificates openssl && \
    rm -rf /var/lib/apt/lists/*

# Add the target
RUN rustup target add $RUST_MUSL_TARGET

# Create a new empty project
WORKDIR /usr/src/service

RUN mkdir -p app
RUN mkdir -p probe
RUN mkdir -p openapi

# copy manifests
COPY Cargo.toml  ./
COPY app/ ./app/
COPY probe/ ./probe/
COPY openapi/ ./openapi/

# Build for release
RUN touch app/main.rs && \
    cargo build -p app --target $RUST_MUSL_TARGET --release

RUN touch probe/main.rs && \
    cargo build -p probe --target $RUST_MUSL_TARGET --release

#TODO: cert and pem creation for SSL between container and ALB

# Final stage
FROM scratch

ARG APP_NAME=orde-rs-service
ARG RUST_MUSL_TARGET=aarch64-unknown-linux-musl

WORKDIR service

# Copy the binary
COPY --from=builder /usr/src/service/target/$RUST_MUSL_TARGET/release/app ./app
COPY --from=builder /usr/src/service/target/$RUST_MUSL_TARGET/release/probe ./probe

# port for the REST API
EXPOSE 443

# Set the entrypoint
ENTRYPOINT ["/service/app"]
