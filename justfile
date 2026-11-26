build:
    cargo build

rbuild:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

no-std:
    cargo build --release --target=riscv32i-unknown-none-elf

test:
    cargo nextest run

clean:
    cargo clean

doc:
    cargo doc

doc-open:
    cargo doc --open
