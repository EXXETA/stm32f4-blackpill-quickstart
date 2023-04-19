# `STM32F401 Black Pill Template`

> A template for the STM32F4xx Black Pill Board

Generate a project with this template:

``` console
$ cargo generate --git https://github.com/SailnMobula/stm32f4-blackpill-quickstart
```

Give your project a name and `cd` into it:

``` console
$ cd my-app
```

Build it

``` console
$ cargo build --release
```

Flash it:

```console
& cargo flash --release --chip STM32F401CEUx
```

Or flash the example it:

```console
& cargo flash --example dht22 --chip STM32F401CEUx
```

You might need to change the chi `STM32F401CEUx` p to your used chip

Debug it

```console
& cargo embed --release
```

In another terminal start gdb

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
