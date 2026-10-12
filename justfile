build:
    cargo build

rbuild:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

clean:
    cargo clean

doc:
    cargo doc

doc-open:
    cargo doc --open

no-std:
    cargo build --release --target=thumbv6m-none-eabi

test:
    cargo test