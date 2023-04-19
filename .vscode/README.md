# VS Code Configuration

Example configurations for debugging programs in-editor with VS Code.  
This directory contains configurations for two platforms:

 - `STM32F401x` via OpenOCD

## Required Extensions

If you have the `code` command in your path, you can run the following commands to install the necessary extensions.

```sh
code --install-extension rust-lang.rust-analyzer
code --install-extension marus25.cortex-debug
```

Otherwise, you can use the Extensions view to search for and install them, or go directly to their marketplace pages and click the "Install" button.

- [Rust Language Server (rust-analyzer)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Cortex-Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug)

## Use

The quickstart comes with two debug configurations.
Both are configured to build the project, using the default settings from `.cargo/config`, prior to starting a debug session.

1. OpenOCD: Starts a debug session for a `STM32F401 Black Pill` board (or any `STM32F401x` running at 25MHz).
   - Follow the instructions above for configuring the build with `.cargo/config` and the `memory.x` linker script.
   - `ITM` output will be written to the Output view `SWO: ITM [port: 0, type: console]` output.

### Git

Files in the `.vscode/` directory are `.gitignore`d by default because many files that may end up in the `.vscode/` directory should not be committed and shared.  
If you would like to save this debug configuration to your repository and share it with your team, you'll need to explicitly `git add` the files to your repository.

```sh
git add -f .vscode/launch.json
git add -f .vscode/tasks.json
git add -f .vscode/*.svd
```

## Customizing for other targets

For full documentation, see the [Cortex-Debug][cortex-debug] repository.

### Device

Some configurations use this to automatically find the SVD file.  
Replace this with the part number for your device.

```json
"device": "STM32F401CEU6",
```

### OpenOCD Config Files

The `configFiles` property specifies a list of files to pass to OpenOCD.

```json
"configFiles": [
    "interface/stlink.cfg",
    "target/stm32f4x.cfg"
],
```

See the [OpenOCD config docs][openocd-config] for more information and the [OpenOCD repository for available configuration files][openocd-repo].

### SVD

The SVD file is a standard way of describing all registers and peripherals of an ARM Cortex-M mCU.  
Cortex-Debug needs this file to display the current register values for the peripherals on the device.  

You can probably find the SVD for your device on the vendor's website.  


For example, the STM32F401 Black Pill board uses an mcu from the `STM32F401x` line of processors.  
All the SVD files for the STM32F4 series are available on [ST's Website][stm32f4].  
Download the [stm32f4 SVD pack][stm32f4-svd], and copy the `STM32F401.svd` file into `.vscode/`.  
This line of the config tells the Cortex-Debug plug in where to find the file.

```json
"svdFile": "${workspaceRoot}/.vscode/STM32F401.svd",
```

For other processors, simply copy the correct `*.svd` file into the project and update the config accordingly.

### CPU Frequency

If your device is running at a frequency other than 25MHz, you'll need to modify this line of `launch.json` for the `ITM` output to work correctly.

```json
"cpuFrequency": 25000000,
```

### Other GDB Servers

For information on setting up GDB servers other than OpenOCD, see the [Cortex-Debug repository][cortex-debug].

[cortex-debug]: https://github.com/Marus/cortex-debug
[stm32f4]: https://www.st.com/en/microcontrollers-microprocessors/stm32f4-series.html
[stm32f4-svd]: https://www.st.com/resource/en/svd/stm32f4_svd.zip
[openocd-config]: http://openocd.org/doc/html/Config-File-Guidelines.html
[openocd-repo]: https://sourceforge.net/p/openocd/code/ci/master/tree/tcl/
