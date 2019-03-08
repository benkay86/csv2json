FROM rust

RUN rustup component add clippy \
 && rustup component add rustfmt \
 && cargo install cargo-outdated \
 && cargo install cargo-audit
# ToDo: Install cargo-tarpaulin for code coverage
