#!/usr/bin/env python3

"""
flasher.py

Automates building, erasing, and flashing firmware to a microcontroller using Cargo and probe-rs.

## Usage

    python flasher.py [-c CHIP] <path_to_elf>

### Arguments

- `-c`, `--chip`      Specify the chip type (default: STM32G431R8)
- `path_to_elf`       Path to the ELF binary to be flashed

### Features

- Cleans the Cargo project
- Erases the microcontroller flash
- Builds the firmware with the 'gdb' profile
- Converts the ELF to HEX and BIN formats
- Downloads the firmware to the microcontroller

### Supported Debug Probes

- J-Link (via probe-rs)
- ST-Link (via probe-rs)
- Black Magic Probe (via arm-none-eabi-gdb)

### Dependencies

- Python 3.x
- cargo
- probe-rs
- arm-none-eabi-objcopy
- arm-none-eabi-gdb (for Black Magic Probe)

---

Author: Julian Bolivar
Date: 2025-07-12
Version: 1.0.0
"""

import os
import subprocess
import sys
from argparse import ArgumentParser
import re

# Define the commands to be executed for different debug probes trough probe-rs
commands_probe_rs = [
    ["cargo", "clean"],
    ["probe-rs", "erase", "--chip", 'chip_id'],
    ["cargo", "build", "--profile", "gdb"],
    ["arm-none-eabi-objcopy", "-O", "ihex", 'path_elf', 'path_hex'],
    ["arm-none-eabi-objcopy", "-O", "binary", 'path_elf', 'path_bin'],
    ["probe-rs", "download", "--chip", 'chip_id', 'path_elf']
]

# Define the commands to be executed for Black Magic Probe
commands_bmp = [
    ["cargo", "clean"],
    ["cargo", "build", "--profile", "gdb"],
    ["arm-none-eabi-objcopy", "-O", "ihex", 'path_elf', 'path_hex'],
    ["arm-none-eabi-objcopy", "-O", "binary", 'path_elf', 'path_bin'],
    ["arm-none-eabi-gdb", "-nx", "--batch", "-ex", "target extended-remote com_p", "-x", "black_magic_probe_flash.scr", 'path_elf']
]

def build_argparser():
    """
    Parse command line arguments.

    :return: command line arguments
    """
    parser = ArgumentParser(
        description="Automate building, erasing, and flashing firmware to an microcontroller using Cargo and probe-rs."
    )
    parser.add_argument("-c", "--chip", required=False, type=str, default="STM32G431R8",
                        help="Specify the chip type. (default: STM32G431R8)")
    parser.add_argument("path_elf", nargs='?', type=str, default=None,
                        help="Path to the ELF binary to be flashed. If not provided, usage instructions will be displayed.")
    return parser


def run_command(args_parser):
    """
    Executes the build, erase, and flash commands using the provided argument parser.

    Args:
        args_parser (ArgumentParser): The argument parser containing user-supplied arguments.

    Exits:
        The script exits with a non-zero code if any command fails or if required arguments are missing.
    """
    args = args_parser.parse_args()
    if args.path_elf is None:
        print("Error: <path_elf> argument is required.\n")
        args_parser.print_help()
        sys.exit(1)
    path = os.path.join(".", args.path_elf)
    path_hex = os.path.join(".", args.path_elf + ".hex")
    path_bin = os.path.join(".", args.path_elf + ".bin")
    path_elf = os.path.join(".", args.path_elf if args.path_elf.endswith(".elf") else args.path_elf + ".elf")
    #os.rename(args.path_elf, path_elf)
    chip = args.chip
    com_port = 'COM1'  # Default COM port
    print(f"Flashing {path_elf} to {chip}")
    # Check if probe-rs can detect a debug probe
    probe_test = ["probe-rs", "list"]
    result = subprocess.run(probe_test, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    probe_detected = True
    if result.returncode == 0 and "J-Link" in result.stdout.decode():
        print("J-Link debug probe detected")
        commands = commands_probe_rs
    elif result.returncode == 0 and "STLink" in result.stdout.decode():
        print("ST-Link debug probe detected")
        commands = commands_probe_rs
    elif result.returncode == 0 and "Black Magic Probe" in result.stdout.decode():
        commands = commands_bmp
        com_match = re.search(r'COM\d+', result.stdout.decode())
        if com_match:
            com_port = com_match.group(0)
        print(f"Black Magic Probe debug probe detected in {com_port}")
    elif result.returncode != 0 or "No debug probes were found" in result.stdout.decode():
        print("No debug probes were found")
        commands = commands_probe_rs  # Default to probe-rs commands
        probe_detected = False
    commands = [list(
        map(lambda x: x.replace('chip_id', chip).replace('path_elf', path_elf).replace('path_hex', path_hex).replace(
            'path_bin', path_bin).replace('com_p',com_port), cmd)) for cmd in commands]
    for cmd in commands:
        if not probe_detected and cmd[0] == "probe-rs":
            print(f"Skipping command: {' '.join(cmd)} as no debug probe was detected.")
            continue
        result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode != 0:
            print(f"Error executing command: {' '.join(cmd)}")
            sys.exit(result.returncode)
        else:
            if cmd[0] == "cargo" and cmd[1] == "build":
                os.rename(path, path_elf)
    print(f"Flashing was successfully to {chip}")


if __name__ == "__main__":

    args_parser = build_argparser()
    run_command(args_parser)
    sys.exit(0)
