release:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

no-std:
    cargo build --release --target=thumbv7em-none-eabi