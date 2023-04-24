# `STM32F401 Embedded Rust Black Pill Template`

> An embedded rust template for the STM32F4xx Black Pill Board

## Prerequisite

You need to have a stable version of `rust` installed. Make sure your version is `> 1.6`

```console
rustup --version
```

You need to install some tools and the correct target:

```console
rustup target add thumbv7em-none-eabihf
```

Install `cargo-generate` to generate a project from this template

```console
cargo install cargo-generate
```

Install cargo `flash` and `embed` utility to flash or debug your program

```console
cargo install cargo-flash
```

```console
cargo install cargo-embed
```

## Use this template

Generate a project with this template:

```console
cargo generate --git https://github.com/EXXETA/stm32f4-blackpill-quickstart.git
```

Give your project a name and `cd` into it:

```console
cd my-app
```

Build it

```console
cargo build --release
```

Flash it:

```console
cargo flash --release --chip STM32F401CEUx
```

Or build and flash the example:

```console
cargo flash --example dht22 --chip STM32F401CEUx
```

You might need to change the chip `STM32F401CEUx` to your used chip, if you use a different board or chip.

You can debug it by starting a debug session on your chip

```console
cargo embed --release
```

And in another terminal start gdb

```console
& arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/hydroponic-stm32f4
& (gdb) target remote :1337
& (gdb) continue
```

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in
the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
