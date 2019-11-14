rustup target add wasm32-unknown-unknown
rustup target add x86_64-pc-windows-msvc
rustup component add rust-src clippy rustfmt rls-preview rust-analysis
cargo install cargo-play cargo-edit watchexec wasm-pack --force
