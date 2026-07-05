build:
    cargo build

build-rel:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

clean:
    cargo clean

doc:
    cargo doc

doc-open:
    cargo doc --open

no-std:
    cargo build --release --target=thumbv7em-none-eabi

test:
    cargo test