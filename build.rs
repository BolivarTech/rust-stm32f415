//! # build.rs
//!
//! This build script is responsible for configuring and compiling C, C++, and assembly source files
//! for the STM32 firmware project using the `cc` crate. It sets up the cross-compiler, adds source
//! and header files, defines macros, sets compiler and linker flags, and ensures Cargo tracks all
//! relevant files for changes. The script is tailored for building STM32 firmware components and
//! integrates the generated object files into the Rust build process, but is can be easy addapted
//! to be used on other vendors MCUs.
//!
//! Author: Julian Bolivar
//! Version: 1.0.0
//! Date: 2025-07-14
use std::fs;
use std::{env};


/// Adds all `.c`, `.cpp`, `.s`, and `.asm` files from the given source path to the provided
/// `cc::Build` builder for compilation.
///
/// # Arguments
///
/// * `builder` - A mutable reference to a `cc::Build` instance used for compiling C/C++ files.
/// * `src_path` - The path to the directory or file containing the C/C++/assembly source files.
///
/// # Details
///
/// This function checks if the given `src_path` is a file or directory. If it is a file and has a
/// supported extension, it is added to the build. If it is a directory, all files with supported
/// extensions in that directory are added to the build process. Supported extensions are `.c`,
/// `.cpp`, `.s`, and `.asm`.
fn add_source_files(builder: &mut cc::Build, src_path: &str) {
    let metadata = fs::metadata(src_path).expect(&format!("can not access {}", src_path));
    if metadata.is_file() {
        let path = std::path::Path::new(src_path);
        if matches!(path.extension().and_then(|s| s.to_str()), Some("c") | Some("cpp") | Some("s") | Some("asm")) {
            println!("Compiling {:?}", path);
            println!("cargo::rerun-if-changed={}", path.display());
            builder.file(&path);
        }
    } else if metadata.is_dir() {
        for entry in fs::read_dir(src_path).expect(&format!("can not read {} folder", src_path)) {
            let path = entry.unwrap().path();
            if path.is_dir() {
                add_source_files(builder, path.to_str().unwrap());
            } else if matches!(path.extension().and_then(|s| s.to_str()), Some("c") | Some("cpp") | Some("s") | Some("asm")) {
                println!("Compiling {:?}", path);
                println!("cargo::rerun-if-changed={}", path.display());
                builder.file(&path);
            }
        }
    }
}


///
/// Adds all native static library files (`.a` files) from the given source path for linking.
///
/// # Arguments
///
/// * `src_path` - The path to the directory or file containing the native static library files.
///
/// # Details
///
/// This function checks if the given `src_path` is a file or directory. If it is a file and has a
/// `.a` extension, it prints the appropriate Cargo instruction to link the static library. If it is
/// a directory, all `.a` files in that directory are processed similarly. This allows integration of
/// prebuilt native libraries into the Rust build process.
///
fn add_native_lib_files(src_path: &str) {
    let metadata = fs::metadata(src_path).expect(&format!("can not access {}", src_path));
    if metadata.is_file() {
        let path = std::path::Path::new(src_path);
        if matches!(path.extension().and_then(|s| s.to_str()), Some("a") ) {
            let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("error:native-lib");
            let lib_name = file_stem.trim_start_matches("lib");
            println!("cargo:rustc-link-lib=static={}", lib_name);
        }
    }
    else if metadata.is_dir() {
        for entry in fs::read_dir(src_path).expect(&format!("can not read {} folder", src_path)) {
            let path = entry.unwrap().path();
            if !path.is_dir() {
                if matches!(path.extension().and_then(|s| s.to_str()), Some("a") ) {
                    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("error:native-lib");
                    let lib_name = file_stem.trim_start_matches("lib");
                    println!("cargo:rustc-link-lib=static={}", lib_name);
                }
            }
        }
    }
}


/// The main entry point for the build script.
///
/// This function configures and compiles C/C++ source files for the project using the `cc` crate.
/// It sets up the cross-compiler, includes source and header files, defines macros, adds assembly files,
/// sets compiler flags, and passes linker arguments to Cargo. The script is tailored for building
/// STM32 firmware components and ensures that all relevant files are tracked for changes.
fn main() {
    // Ensure the OUT_DIR environment variable is set by Cargo
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
    // Set the cross-compiler for the ARM architecture with hard-float ABI
    env::set_var("CC_arm-none-eabi", "C:\\Program Files (x86)\\Arm GNU Toolchain \
                 arm-none-eabi\\14.3 rel1\\bin\\arm-none-eabi-gcc");  // Set the path to the ARM GCC compiler
    //env::set_var("CC_arm-none-eabi", "C:\\ST\\STM32CubeCLT_1.18.0\\GNU-tools-for-STM32\
    //             \\bin\\arm-none-eabi-gcc.exe");   // Set the path to the STM32 ARM GCC compiler
    //env::set_var("CC_arm-none-eabi","arm-none-eabi-gcc"); // Set the first in PATH ARM GCC compiler found
    println!("cargo:rustc-link-search=native={}", out_dir);

    // Create a new cc::Build instance
    let mut builder: cc::Build = cc::Build::new();

    //1. set the cross compiler
    //builder.compiler(env::var("CC_arm-none-eabi").unwrap());
    builder.compiler("arm-none-eabi-gcc");

    //2. Add all .c, .cpp, .s and .asm files from the specified directories
    let src_paths: [&str; 0] = [
        // "cpp_src/c-stm32f415rgt6/Core",
        // "cpp_src/c-stm32f415rgt6/Drivers"
    ];
//        "cpp_src/c-stm32f415rgt6/Core",
//        "cpp_src/c-stm32f415rgt6/Drivers"
//    ];
    for src_path in src_paths.iter() {
        add_source_files(&mut builder, src_path);
    }

    //3. Add all C/C++ include files (.h files)
    let include_paths:[&str; 0] = [
//        "cpp_src/c-stm32f415rgt6/Core/Inc",
//        "cpp_src/c-stm32f415rgt6/Drivers/STM32F4xx_HAL_Driver/Inc",
//        "cpp_src/c-stm32f415rgt6/Drivers/STM32F4xx_HAL_Driver/Inc/Legacy",
//        "cpp_src/c-stm32f415rgt6/Drivers/CMSIS/Device/ST/STM32F4xx/Include",
//        "cpp_src/c-stm32f415rgt6/Drivers/CMSIS/Include"
    ];
    for include_path in include_paths.iter() {
        builder.include(include_path);
        println!("cargo:rerun-if-changed={}", include_path);
    }

    // Include native static libraries
    let lib_paths: [&str; 0] = [
        //"C:\\Program Files (x86)\\Arm GNU Toolchain arm-none-eabi\\14.3 rel1\\arm-none-eabi\\lib",
        //"C:\\Program Files (x86)\\Arm GNU Toolchain arm-none-eabi\\14.3 rel1\\arm-none-eabi\\lib\\arm\\v5te\\hard"
        //"C:\\ST\\STM32CubeCLT_1.18.0\\GNU-tools-for-STM32\\arm-none-eabi\\lib"
        //"C:\\ST\\STM32CubeCLT_1.18.0\\GNU-tools-for-STM32\\arm-none-eabi\\lib\\arm\\v5te\\hard"
    ];
    if !lib_paths.is_empty() {
        for lib_path in lib_paths.iter() {
            println!("cargo:rustc-link-search=native={}", lib_path);
            add_native_lib_files(lib_path);
        }
    }

    //4. Add Define macros, -D (optional)
    let defines = [
        "DEBUG",
        "USE_HAL_DRIVER",
        "STM32F415xx"
    ];
    for define in defines.iter() {
        builder.define(define, None);
    }

    //5. Add compiler flags
    let compiler_flags = [
        "-mcpu=cortex-m4",
        "-mthumb",
        "-mfpu=fpv4-sp-d16",
        "-mfloat-abi=hard",
        "-std=gnu11",
        "-g3",
        "-O0",
        "-ffunction-sections",
        "-fdata-sections",
        "-Wall",
        "-fstack-usage",
        //"-fcyclomatic-complexity"   // Flag not supported by the original compiler ARM 14.3
    ];
    for flag in compiler_flags.iter() {
        builder.flag(flag);
    }

    //6 . Add linker flags
    let linker_flags = [
        //"-nostdlib",
        //"--specs=nano.specs",
        "--specs=nosys.specs",
        "-Wl,--gc-sections",
        "-Wl,--verbose"
    ];
    for flag in linker_flags.iter() {
        println!("cargo:rustc-link-arg={}", flag);
    }

    //7. generate object files for C files
    // builder.compile("stm32_c_drivers");
    let object_files = builder.compile_intermediates();

    //8. this tells the cargo to pass each object file directly to the linker
    for obj_file in &object_files {
        println!("cargo:rustc-link-arg={}", obj_file.display());
    }

    //8.b this is an alternative way to pass the object files to the linker as a static library
    //     NOTE: This can produce issues with weak symbols, and interrupt handlers are not replaced
    //            by the strong symbols, and default handlers are used instead.
    //            You can verify this by checking using the `cargo nm` command, that will show
    //            the 'w' flag and not the 'T' flag to indicate that the strong implementation was
    //            linked.
    //builder.compile("stm32_c_drivers");

    //8.c informs Cargo to pass -lstm32_c_drivers to the linker, which makes the linker look for
    // libstm32_c_drivers.a and link it.
    //println!("cargo:rustc-link-lib=static=stm32_c_drivers");
}
