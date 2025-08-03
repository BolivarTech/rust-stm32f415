# Visual Studio Code Rust Embedded Template

This project provides a minimal template for starting a `#![no_std]` Rust embedded application. It is designed to be a 
starting point for bare-metal or embedded development without the Rust standard library.

This template is to be used in Visual Studio Code and it is based on the plain template
[Rust Embedded Template](https://github.com/BolivarTech/rust-embedded-template).

This template is intended for creating embedded applications using Rust in a bare-metal environment on an MCU where no 
crate exists that implements the HAL (Hardware Abstraction Layer) for your specific microcontroller. Due to time 
constraints, you may not have time to implement your own HAL, so this template allows you to use existing C/C++ HALs 
through the Foreign Function Interface (FFI).

With this template, you can also incorporate and use standard embedded libraries like `embedded-hal`, `cortex-m`, and 
`cortex-m-rt`, and include external C/C++ source files if needed, using the Foreign Function Interface (FFI).

## Directory and Files Structures

├── cpp_src/&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;# Optional C/C++ source files (if needed)  
├── src/&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;# Rust source files  
│&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;└── main.rs&nbsp;&nbsp;&nbsp;&nbsp;# Main entry point with custom _start function and panic handler  
├── doc/&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;# Template documentation   
├── Cargo.toml&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;# Project configuration and dependencies  
├── README.md&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;# This file  
└── LICENSE&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;# MIT License file  

## Features

- Basic project structure for Rust embedded applications
- `#![no_std]` and `#![no_main]` setup
- Minimal panic handler
- Ready for further embedded development
- Optional C/C++ source files support

This template implements a memory allocator using the low-stack allocation strategy, which is suitable for embedded systems 
to prevent stack overflow issues, because the stack is allocated at the beginning of the RAM and grows downwards; if the
stack grows too large, it will reach the lowe RAM address, and if try to write bellow that address, it will cause an 
application panic and trigger the *HardFault_Handler*.

This strategy is implemented in the linker script [linker_low_stack.ld](./linker_low_stack.ld) and in the 
[startup_stm32g431.rs](./src/startup_stm32g431.rs) file.  

## Stack Guard

This template includes a stack guard mechanism to help detect and prevent stack overflows, which are critical in 
embedded systems. The stack guard is implemented by reserving a specific memory region at the end of the stack in the
linker script (`linker_low_stack.ld`). If the stack pointer exceeds this region (due to excessive stack usage), a 
memory access violation occurs and the function `stack_guard_corrupted()` will return TRUE; providing to the system 
a mechanism to catch stack overflows early and handle them gracefully, rather than causing unpredictable behaviors.

The stack guard setup is coordinated between the linker script and the startup code (`src/startup_stm32g431.rs`), 
ensuring that the stack does not overwrite critical memory regions.

## Usage

To use this template, follow these steps:

### Use Cargo Generate

This is the recommended way to create a new project based on this template. It allows you to quickly scaffold a new Rust
embedded project with the necessary files and structure.

1. **Install `cargo-generate`:**
   If you haven't already, install `cargo-generate`:
   ```bash
   cargo install cargo-generate
   ```
2. **Generate a new project:**
   ````bash
   cargo generate --git https://github.com/BolivarTech/rust-embedded-template-vsc.git --name myproject

   ````

### Clone Git Repository

If you prefer to clone the repository directly, you can do so. This method is useful if you want to explore the code or
make modifications before starting your own project.

1. **Clone the repository:**
   ```
   git clone https://github.com/BolivarTech/rust-embedded-template-vsc.git
   cd rust-embedded-template-vsc
   ```

2. **Build the project:**
   ```
   cargo build
   ```

   > **Note:** Running `cargo test` is not supported in `#![no_std]` projects.

3. **Flash or run on your target hardware**  
   (Refer to your hardware or emulator documentation for details.)

## Documentation

For more detailed information on how to set up and use this template, refer to the [How-to Guide](doc/howto_detailed_setup.md).

For information about required Visual Studio Code extensions and plugins, see the [VS Code Extensions Guide](VSCODE_EXTENSIONS.md).

Additional documentation can be found in the [How to C/C++ FFI](doc/how_to_cpp_ffi.md) guide, which explains how to 
interface Rust with C/C++ code using the Foreign Function Interface (FFI).

the [How to SVD2Rust to PAC](doc/how_to_svd2rust_to_PAC.md) guide, which explains how to generate a Peripheral Access 
Crate (PAC) from SVD files and use it in your Rust embedded application.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE.md) file for details.
