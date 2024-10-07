# Rust Embassy Examples (BBC Microbit v2)

> Install Rust: https://www.rust-lang.org/tools/install
>  
> $ rustup component add llvm-tools-preview  
> $ cargo install cargo-binutils  

![](https://cdn.sanity.io/images/ajwvhvgo/production/69cda3f409b82d272fd8cc2ad9e95d731dbe3865-1688x734.png?w=653&q=80&fit=max&auto=format)

## Quick Start

```
$ cd nrf52833
$ cargo build --bin blinky --release
$ cargo run --bin blinky --release

# Generate bin firmware
$ cargo objcopy --release -- -O binary firmware.bin
```
