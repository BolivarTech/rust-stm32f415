# Rust Embedded Detailed Setup

## Install Rust Toolchain

For installation instructions, see [Rust Toolchain Install Guide](https://www.rust-lang.org/tools/install).

## Install Cargo Utils

  ```bash
  rustup target install <TARGET>  # Example: rustup target install thumbv7em-none-eabihf
  cargo install cargo-binutils cargo-generate cargo-modules form svd-vector-gen svd2rust cargo-llvm-cov
  cargo install probe-rs-tools --locked
  rustup component add llvm-tools-preview
  ```
## Read ELF file content

To read the content of an ELF file, you can use the `cargo readobj` command or `cargo objdump` command.
These commands will help you inspect the ELF file's headers and sections.

```bash
cargo objdump -- -h <ELF_FILE>
```


```bash
cargo readobj -- -h <ELF_FILE>
cargo readobj -- -S <ELF_FILE>
```

Print all sections and their details:
```bash
cargo readobj -- -all <ELF_FILE>
```

Print specific section details, for example, the `.data` section:

```bash
cargo readobj -- -x .text <ELF_FILE>
cargo readobj -- -x .rodata <ELF_FILE>
cargo readobj -- -x .data <ELF_FILE>
cargo readobj -- -x .bss <ELF_FILE>
```

## ELF (Executable and Linkable Format) File Structure

ELF File  
  ├─ ELF header&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(Contains Metadata about the file)  
  ├─ Program Header (Provide information about the segments that need to be  
  │&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  &nbsp;loaded into memory for the program run correctly)  
  ├─ Sections  
  │&nbsp;&nbsp;&nbsp;&nbsp;├─ .text&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(Executable code)  
  │&nbsp;&nbsp;&nbsp;&nbsp;├─ .data&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(Initialized data)  
  │&nbsp;&nbsp;&nbsp;&nbsp;├─ .bss&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(Uninitialized data)  
  │&nbsp;&nbsp;&nbsp;&nbsp;├─ .rodata&nbsp;&nbsp;&nbsp;(Read-Only data)  
  │&nbsp;&nbsp;&nbsp;&nbsp;├─ .symtab&nbsp;&nbsp;(Symbol table)  
  │&nbsp;&nbsp;&nbsp;&nbsp;├─ .debug&nbsp;&nbsp;&nbsp;&nbsp;(Debugging information for debuggers like GDB)  
  │&nbsp;&nbsp;&nbsp;&nbsp;└─ .strtab&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(String table)  
  └─ Section header table (Describe sections within the file)  

## Setup Cargo Environment:

At the project's root, create file and directory structure as follows:

```bash
mkdir -p .cargo && touch .cargo/config.toml
```

In the file create this template.

```toml
[build]
target = "thumbv7em-none-eabihf" # Specify the target architecture

[target.thumbv7em-none-eabihf]
#linker = "arm-none-eabi-ld"  # Uncomment if you want to use a custom linker
rustflags = [
    "-C", "link-arg=-Tlinker.ld",  # Linker script
    "-C", "link-arg=-Map=output.map", # Map file for debugging
    "-C", "link-arg=-L./lib", # Additional library path
]
```
This will set the target architecture and specify the linker script and additional flags for the build process.

## Create Linker Script

On the projec's root level create the file: *linker.ld*  
**Note:** this can be any name but must be the same in the *config.toml*

The linker script defines the memory layout of the target device. Here is a basic example for an ARM Cortex-M target:
[linker.ld](../linker.ld)  

## Create Vector Table

One way to create a vector table is to use the `svd-vector-gen` crate; this tool scans the current directory for
ARM Cortex-M compatible SVD files and automatically generates:  

- Vector Table File (vector_<mcu>.txt):
  - Contains the vector table for the specified microcontroller with system exceptions and interrupt handlers.
  - Format: A static VECTOR_TABLE Rust array with Option<unsafe fn()> entries for each vector, including system handlers and IRQs.
- Device-Specific Linker Script (device_<mcu>.x):
  - Defines PROVIDE entries for all interrupts as:
  - PROVIDE(<IRQ_NAME> = default_handler); 

This facilitates linking during firmware development.

### Install svd-vector-gen

```bash
cargo install svd-vector-gen
```
### Generate Vector Table
Ensure that the directory contains valid SVD files.
```bash
svd-vector-gen 
```
### Generate Vector Table for Specific MCU
For example on the STM32G431.svd:

Generated Files:
  - vector_STM32G431.txt: Contains the vector table code for the startup_STM32G431.rs file.
  - device_STM32G431.x: Contains the linker script PROVIDE attribute.

## Create Startup File
The startup file is responsible for initializing the microcontroller and setting up the vector table. It typically includes the following:
- Reset handler
- Default exception handlers
- Interrupt Service Routines (ISRs)
- Initialization code for peripherals
- Stack and heap initialization
- Global variable initialization

You can create a startup file manually or use a template from the `svd-vector-gen` output. The startup file should be named according to the microcontroller, e.g., `startup_STM32G431.rs`.  
The file [startup_STM32G431.rs](../src/startup_stm32g431.rs) is an example of a startup file for the STM32G431 microcontroller. It includes the vector table and default handlers for exceptions and interrupts.

## Setup Cargo Embedded Tools

To set up the embedded tools, you can use the `probe-rs-tools` package, which provides tools for working with embedded devices.

```bash
cargo install probe-rs-tools --locked
```

To get the list of available chips, you can use the following command:

```bash
probe-rs chip list | grep -i <CHIP_NAME>
```

or in PowerShell:

```powershell
probe-rs chip list | Select-String -Pattern "<CHIP_NAME>"
```
### Flash the Firmware

To flash the firmware to the target device, you can use the `probe-rs` command-line tool. Make sure your device is connected and recognized by the system.

```bash
cargo flash --chip <CHIP_NAME>
```
You can also use the `probe-rs` command directly:

```bash
probe-rs flash --chip <CHIP_NAME> --file <FIRMWARE_FILE>
```

In the file ./.cargo/config.toml, you can specify the chip and other options for flashing:

In the section `[target.thumbv7em-none-eabihf]`, you can add the `runner` option to specify the command to run after flashing:
```toml
runner = "probe-rs run --chip STM32G431R8"
```

On this way, you can run the firmware directly after flashing it to the device.

```bash
cargo run
```

#### Using Custoimized Cargo run

You can customize the `cargo run` command to include additional options for flashing and running the firmware. For example, you can create a custom script or use the `cargo run` command with specific flags.

```toml
runner = "python flasher.py --chip STM32G431R8"  # Custom runner script for building, flashing and running
```

This allows you to have more control over the build and flash process, and you can include additional logic in the `runner.py` script to handle different scenarios.

The `flasher.py` script receive the path to the firmware file as an argument and can be used to build, flash, and run the firmware on the target device.


## Debugging with GDB

Debugging embedded firmware can be done using GDB (GNU Debugger) with the help of `probe-rs` or other GDB servers
compatible with embedded devices.

This depends on the used IDE; for example, in Visual Studio Code, you can use the `CodeLLDB` extension or `Cortex-Debug`
extension for debugging embedded applications.

On JetBrains IDEs, you can use the `Embedded GDB Server` plugin in Clion, which provides support for debugging embedded
applications.

**Note:** At the moment of this writing, JetBrain RustRover does not support the `Embedded GDB Server`.

To debug the firmware using GDB, you need a GDB server running. First, ensure that you have GDB installed and configured
for your target architecture; and the IDE configured to use the GDB server.

For example, you can use the `probe-rs` GDB server to debug the firmware on Visual Studio Code as explained in the 
[probe-rs Debuggin documentation](https://probe.rs/docs/tools/debugger/).

