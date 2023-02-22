FROM --platform=${BUILDPLATFORM} rust:1 AS build

WORKDIR /app

ARG TARGETARCH

ENV OPENSSL_DIR="/etc/ssl" \
    OPENSSL_LIB_DIR="/usr/lib" \
    OPENSSL_INCLUDE_DIR="/usr/include"

RUN case "$TARGETARCH" in \
    "arm64") echo aarch64-unknown-linux-musl > /target;; \
    "amd64") echo x86_64-unknown-linux-musl > /target ;; \
    *) exit 1 ;; \
    esac
RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev musl-tools
RUN rustup default nightly
RUN rustup target add $(cat /target)
RUN cargo init

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release --target $(cat /target)
RUN rm -rf src

COPY src ./src

RUN touch src/main.rs
RUN cargo build --release --target $(cat /target)
RUN mv ./target/$(cat /target)/release/tums ./tums


FROM scratch

COPY --from=build /app/tums /tums

CMD [ "/tums" ]