FROM cffnpwr/rust-mold

RUN mold -run rustup default nightly && \
    mold -run rustup update && \
    mold -run rustup component add clippy rustfmt && \
    mold -run cargo install cargo-watch cargo-edit

WORKDIR /app

COPY . .