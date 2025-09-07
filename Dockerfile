###############################################################################
# 1. Common build environment

ARG BASE_IMAGE=rust:1.84.0

FROM ${BASE_IMAGE} AS base_builder

# LABEL authors="operations@opentensor.ai" \
#   vendor="Opentensor Foundation" \
#   title="opentensor/subtensor" \
#   description="Opentensor Subtensor Blockchain" \
#   documentation="https://docs.bittensor.com"

# Rust targets
RUN rustup update stable && \
  rustup target add wasm32-unknown-unknown --toolchain stable

# Build prerequisites
ENV RUST_BACKTRACE=1
RUN apt-get update && \
  apt-get install -y --no-install-recommends \
  curl build-essential protobuf-compiler clang git pkg-config libssl-dev && \
  rm -rf /var/lib/apt/lists/*

COPY . /build
WORKDIR /build

###############################################################################
# 2. Production build stage

FROM base_builder AS prod_builder

# Build the production binary (profile defined in Cargo.toml)
RUN cargo build -p jungochain-node --profile production --features "metadata-hash" --locked

# Verify the binary was produced
RUN test -e /build/target/production/jungochain-node

###############################################################################
# 3. Final production image (hardened)

FROM ${BASE_IMAGE} AS jungochain

COPY --from=prod_builder /build/*.json ./
COPY --from=prod_builder /build/target/production/jungochain-node /usr/local/bin/

EXPOSE 30333 9933 9944

ENTRYPOINT ["/usr/local/bin/jungochain-node"]
