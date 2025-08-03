# Visual Studio Code Extensions for Rust Embedded Template

This document lists all the Visual Studio Code extensions/plugins required and recommended for working with this Rust embedded template project.

## Required Extensions

These extensions are **essential** for the template to work properly:

### 1. Rust Language Support

#### rust-analyzer
- **Extension ID**: `rust-lang.rust-analyzer`
- **Description**: Official Rust language server providing intelligent code completion, error checking, refactoring, and more
- **Why needed**: Core Rust language support, syntax highlighting, IntelliSense, and error detection
- **Installation**: 
  ```
  code --install-extension rust-lang.rust-analyzer
  ```

### 2. Embedded Debugging

#### Cortex-Debug
- **Extension ID**: `marus25.cortex-debug`
- **Description**: ARM Cortex-M GDB debugger support for Visual Studio Code
- **Why needed**: Enables debugging of ARM Cortex-M microcontrollers (STM32G431R8 in this template)
- **Features**:
  - J-Link debugger support
  - ST-Link debugger support  
  - Black Magic Probe support
  - SVD file support for peripheral register viewing
  - Real-time variable monitoring
- **Installation**:
  ```
  code --install-extension marus25.cortex-debug
  ```

## Highly Recommended Extensions

These extensions significantly improve the development experience:

### 3. C/C++ Support (for FFI)

#### C/C++
- **Extension ID**: `ms-vscode.cpptools`
- **Description**: IntelliSense, debugging, and code browsing for C/C++
- **Why needed**: This template supports C/C++ FFI, so C/C++ language support is useful for mixed projects
- **Installation**:
  ```
  code --install-extension ms-vscode.cpptools
  ```

### 4. Assembly Language Support

#### ARM Assembly
- **Extension ID**: `dan-c-underwood.arm`  
- **Description**: Syntax highlighting for ARM assembly language
- **Why needed**: When working with low-level embedded code, you may need to read or write assembly
- **Installation**:
  ```
  code --install-extension dan-c-underwood.arm
  ```

### 5. Linker Script Support

#### Linker Script
- **Extension ID**: `zixuanwang.linkerscript`
- **Description**: Syntax highlighting for linker scripts (.ld files)
- **Why needed**: This template includes custom linker scripts (`linker.ld`, `linker_low_stack.ld`)
- **Installation**:
  ```
  code --install-extension zixuanwang.linkerscript
  ```

## Optional but Useful Extensions

### 6. File Management

#### File Utils  
- **Extension ID**: `sleistner.vscode-fileutils`
- **Description**: Convenient file operations
- **Why useful**: Easier file management in embedded projects

#### Path Intellisense
- **Extension ID**: `christian-kohler.path-intellisense`
- **Description**: Autocomplete for file paths
- **Why useful**: Helpful when including external files or configuring paths

### 7. Git Integration

#### GitLens
- **Extension ID**: `eamodio.gitlens`
- **Description**: Enhanced Git capabilities
- **Why useful**: Better version control visualization and blame information

### 8. Documentation

#### Markdown All in One
- **Extension ID**: `yzhang.markdown-all-in-one`
- **Description**: Markdown language features  
- **Why useful**: For editing project documentation and README files

#### Better Comments
- **Extension ID**: `aaron-bond.better-comments`
- **Description**: Improved comment highlighting
- **Why useful**: Better code documentation in embedded projects

### 9. Code Quality

#### Error Lens
- **Extension ID**: `usernamehw.errorlens`
- **Description**: Inline error and warning display
- **Why useful**: Immediate visibility of compilation errors and warnings

#### Bracket Pair Colorizer
- **Extension ID**: `coenraads.bracket-pair-colorizer`
- **Description**: Colorizes matching brackets
- **Why useful**: Easier code navigation in complex embedded code

## System Requirements

### External Tools Required

Before installing VS Code extensions, ensure these tools are installed:

#### 1. Rust Toolchain
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add embedded target
rustup target add thumbv7em-none-eabihf
```

#### 2. ARM GNU Toolchain
- **Download**: [ARM GNU Toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
- **Purpose**: Provides `arm-none-eabi-gdb` and other ARM development tools
- **Version**: 14.3 rel1 or later recommended

#### 3. Probe-rs Tools
```bash
cargo install probe-rs-tools --locked
```

#### 4. Debug Probe Software

Choose based on your hardware:

**For J-Link:**
- Download and install [SEGGER J-Link Software](https://www.segger.com/downloads/jlink/)

**For ST-Link:**
- Install [STM32CubeProgrammer](https://www.st.com/en/development-tools/stm32cubeprog.html)
- Or use probe-rs (already installed above)

**For Black Magic Probe:**
- No additional software needed (uses arm-none-eabi-gdb directly)

## Installation Script

You can install all required extensions at once using this command:

```bash
# Required extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension marus25.cortex-debug

# Recommended extensions  
code --install-extension ms-vscode.cpptools
code --install-extension dan-c-underwood.arm
code --install-extension zixuanwang.linkerscript

# Optional extensions
code --install-extension sleistner.vscode-fileutils
code --install-extension christian-kohler.path-intellisense
code --install-extension eamodio.gitlens
code --install-extension yzhang.markdown-all-in-one
code --install-extension aaron-bond.better-comments
code --install-extension usernamehw.errorlens
```

## Extension Configuration

### Workspace Settings

The template includes these pre-configured settings in `.vscode/settings.json`:

```json
{
    "cortex-debug.armToolchainPath": "C:\\Program Files (x86)\\Arm GNU Toolchain arm-none-eabi\\14.3 rel1\\bin",
    "cortex-debug.armToolchainPrefix": "arm-none-eabi",
    "cortex-debug.JLinkGDBServerPath": "C:\\Program Files\\SEGGER\\JLink\\JLinkGDBServer.exe"
}
```

### Launch Configurations

The template provides multiple debug configurations:

1. **Cortex Debug** - Direct J-Link debugging
2. **Flash & Debug with flasher.py** - Automated build and flash with J-Link
3. **Flash & Debug with probe-rs** - Using probe-rs as GDB server
4. **Flash & Debug with Black Magic Probe** - Black Magic Probe support

## Troubleshooting

### Common Issues

1. **rust-analyzer not working**:
   - Ensure Rust is properly installed: `rustup --version`
   - Restart VS Code after installing Rust

2. **Cortex-Debug can't find ARM toolchain**:
   - Update the `cortex-debug.armToolchainPath` in settings.json
   - Verify ARM toolchain installation

3. **Debug probe not detected**:
   - Run `probe-rs list` to check probe detection
   - Verify debug probe drivers are installed
   - Check USB connections

4. **Build errors**:
   - Ensure target is added: `rustup target add thumbv7em-none-eabihf`
   - Check Cargo.toml dependencies

## Verification

After installing extensions, verify the setup:

1. **Open the project** in VS Code
2. **Check Extensions tab** - ensure all required extensions are enabled
3. **Run the "Detect debug probe" task** to verify hardware detection
4. **Try building** with Ctrl+Shift+P → "Tasks: Run Task" → "rust: cargo build embedded"
5. **Test debugging** by pressing F5

## Support

If you encounter issues with any extensions:

1. Check the extension's documentation in VS Code Extensions tab
2. Verify system requirements are met
3. Check VS Code's Output panel for error messages
4. Consult the template's [detailed setup guide](doc/howto_detailed_setup.md)

---

**Last Updated**: July 26, 2025  
**Template Version**: Compatible with rust-embedded-template-vsc v1.0+
