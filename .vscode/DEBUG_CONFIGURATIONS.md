# Debug Launch Configurations

This project now includes multiple debug launch configurations to support different debugging workflows and hardware setups.

## Available Launch Configurations

### 1. Cortex Debug (Original)
- **Name**: "Cortex Debug"
- **Description**: Direct J-Link debugging without using flasher.py
- **Pre-launch**: Builds the project using Cargo
- **Best for**: Quick debugging sessions when firmware is already flashed

### 2. Flash & Debug with flasher.py
- **Name**: "Flash & Debug with flasher.py"  
- **Description**: Uses flasher.py to build, erase, and flash firmware before debugging
- **Pre-launch**: Runs flasher.py script which handles the complete build and flash process
- **Best for**: Complete development workflow with automatic flashing
- **Supported probes**: J-Link, ST-Link (via probe-rs)

### 3. Flash & Debug with probe-rs
- **Name**: "Flash & Debug with probe-rs"
- **Description**: Uses flasher.py for flashing and probe-rs as GDB server
- **Pre-launch**: Flashes with flasher.py and starts probe-rs GDB server
- **Best for**: When you prefer probe-rs as the GDB server over J-Link GDB server
- **Port**: localhost:1337

### 4. Flash & Debug with Black Magic Probe
- **Name**: "Flash & Debug with Black Magic Probe"
- **Description**: Support for Black Magic Probe debugging
- **Pre-launch**: Uses flasher.py for build and flash process
- **Best for**: When using Black Magic Probe hardware
- **Serial Port**: COM1 (auto-detected by flasher.py)

## Available Tasks

### Build Tasks
- **rust: cargo build embedded**: Standard Cargo build
- **Flash with flasher.py**: Runs the flasher.py script
- **Flash with probe-rs and start GDB server**: Combines flashing with GDB server startup

### Utility Tasks
- **Start probe-rs GDB server**: Starts probe-rs GDB server on port 1337
- **Detect debug probe**: Lists all connected debug probes

## Usage Instructions

1. **Choose the appropriate launch configuration** based on your hardware and workflow
2. **Connect your debug probe** (J-Link, ST-Link, or Black Magic Probe)
3. **Press F5** or use the Run and Debug panel to start debugging
4. The selected configuration will automatically:
   - Build the project
   - Flash the firmware (if using flasher.py configurations)
   - Start the debug session

## Troubleshooting

- If you get timeout errors, try the "Flash & Debug with flasher.py" configuration
- Use "Detect debug probe" task to verify your probe is connected
- Check the DEBUG CONSOLE output for detailed error messages
- Ensure your probe drivers are properly installed

## Hardware Requirements

- **J-Link**: Requires SEGGER J-Link software installation
- **ST-Link**: Supported via probe-rs
- **Black Magic Probe**: Uses arm-none-eabi-gdb directly
- **Target**: STM32G431R8 (configurable in tasks and launch configurations)
