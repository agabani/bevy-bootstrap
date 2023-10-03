set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# help
help:
  @just --list

# build wasm
build-wasm:
  @cargo build --release --target wasm32-unknown-unknown
  @wasm-bindgen --out-dir wasm --out-name bevy_bootstrap --target web target/wasm32-unknown-unknown/release/bevy-bootstrap.wasm

# format
format:
  @cargo fmt

# install
install: install-wasm-bindgen install-wasm-server install-wasm-target

# install wasm bindgen
install-wasm-bindgen:
  @cargo install wasm-bindgen-cli

# install wasm server
install-wasm-server:
  @cargo install basic-http-server

# install wasm target
install-wasm-target:
  @rustup target add wasm32-unknown-unknown

# lint
lint:
  @cargo clippy

# run
run:
  @cargo run

# run dev
run-dev:
  @cargo run --features dev

# run wasm
run-wasm: build-wasm
  @basic-http-server wasm

# test
test:
  @cargo test
