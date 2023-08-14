# https://github.com/casey/just

default: build test lint

build:
    cargo build

test:
    RUST_LOG=trace cargo nextest run

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
