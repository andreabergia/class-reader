# https://github.com/casey/just

default: build test build-wasm lint

build:
    cargo build

test:
    RUST_LOG=trace cargo nextest run

build-wasm:
    cargo build --features wasm
    wasm-pack build --features wasm
    cd pkg && cat package.json | sed 's/"name": "class-reader"/"name": "@andreabergia\/class-reader"/' > package2.json && mv package2.json package.json

test-verbose:
    RUST_LOG=trace cargo nextest run --no-capture

lint:
    cargo clippy --fix --allow-dirty --allow-staged

clean:
    cargo clean

fmt:
    cargo +nightly fmt

generate-test-classes:
    cd ./tests/resources && rm -f *.class && ./compile.sh

miri:
    cargo clean
    MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-report-progress" cargo +nightly miri test

find-unused-dependencies:
    cargo +nightly udeps --all-targets

publish-npm:
    cd pkg && npm publish --access=public
