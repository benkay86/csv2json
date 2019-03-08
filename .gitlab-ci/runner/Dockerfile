FROM rust

RUN rustup component add clippy \
 && rustup component add rustfmy \
 && cargo install cargo-outdated \
 && cargo install cargo-audit
# ToDo: Install cargo-tarpaulin for code coverage
