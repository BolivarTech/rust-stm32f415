# How to use SVD2Rust to generate PAC

The `svd2rust` generates Rust register maps (structs) from SVD files.

The PAC (Peripheral Access Crate) is a Rust crate that provides safe access to the hardware registers
of a microcontroller.

It is generated from the SVD files provided by the microcontroller vendor and provides direct access to all peripherals' 
hardware registers. 

## Rust Application, HAL and PAC Crate

`Rust Application`  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;||  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\\/  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`HAL Crate`  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;||    
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\\/  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`PAC Crate`  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;||  
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;\\/  
`Physical Hardware`

## Install svd2rust
```bash
cargo install svd2rust
cargo install form
```

## Generate PAC Crate

Download the SVD file for your microcontroller from the vendor's website.

Follow the steps in the svd2rust [documentation](https://docs.rs/svd2rust/latest/svd2rust/) page to create the PAC crate
for your target architecture.

## PAC Crate Usage

In your `Cargo.toml`, add the generated PAC crates as a dependency and use it in your Rust application to access the
register lever abstraction level.

```toml
[dependencies]
your_pac_crate_name = { path = "../path/to/your/pac/crate" }
```

You can check the crate's dependency tree with the command:
```bash
cargo tree
```
