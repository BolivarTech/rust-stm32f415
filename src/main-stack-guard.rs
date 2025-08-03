#![no_std]
#![no_main]
#![cfg_attr(test, no_main)]

//! # STM32G431 Stack Guard Example
//!
//! This program demonstrates a simple stack guard mechanism for embedded Rust
//! on STM32G431 microcontrollers. It includes a recursive function to test
//! stack overflow, a stack guard region, and a function to check for stack
//! corruption.
//!
//! ## Features
//! - Recursive function to test stack overflow
//! - Stack guard region in a custom linker section
//! - Function to check if the stack guard is corrupted
//! - Minimal panic handler
//!
//! ## Usage
//! The `main` function runs a loop that repeatedly calls the recursive test
//! function and checks the stack guard. If the guard is violated, the loop
//! breaks to prevent further execution.

mod startup_stm32g431;

use core::panic::PanicInfo;
use core::ptr;


/// Recursively increments `i` until it reaches `limit`.
/// Used to test stack overflow.
///
/// # Arguments
/// * `i` - Current value
/// * `limit` - Limit to reach
///
/// # Returns
/// The final value of `i`.
fn test(mut i: i32, limit: i32) -> i32 {
    i += 1;
    if (limit < 0) || (i < limit) {
        return test(i, limit)
    }
    i
}

/// Entry point of the program.
///
/// Runs a loop that calls the recursive test function and checks the stack guard.
/// If the stack guard is corrupted, the loop breaks.
#[no_mangle]
pub extern "C" fn main() -> ! {
    
   
    static mut I: i32 = 0;

    static _ARREGLO: [u32;5] = [1,2,3,4,5];

    let _message = "Hello, World!";

    let _r = test(0,30);  // To test Stack Overflow

    unsafe { I = _r; }

    loop{
        unsafe {

            let _r = test(0,I);  // To test Stack Overflow

            // Check the stack guard
            if stack_guard_corrupted() {
                // If the stack guard is violated, handle the error
                // For example, you could log an error message or reset the system
                // Here we just loop forever to maintain the never type
                loop {
                    // Dummy operation to prevent optimization and debug the stack guard
                    I += 1;

                    // Check if the counter has reached a certain value
                    if I > 10 {
                        // Reset the counter
                        I = 0;
                    }
                }
            }

            // Increment the counter
            I += 1;

            // Check if the counter has reached a certain value
            if I > 3000 {
                // Reset the counter
                I = 0;
            }
        }
        // Optionally, you could add a delay here to slow down the loop
        // For example, using a busy-wait loop or a delay function
    }
}

/// Panic handler for the program.
///
/// Loops forever on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}

// Stack guard pattern used to detect stack overflow.
use startup_stm32g431::STACK_GUARD_PATTERN;

/// Reserve the Stack guard region placed in a custom linker section.
///
/// This region is checked for corruption to detect stack overflows.
#[used]
//#[no_mangle]
#[link_section = ".stack_guard"]
static STACK_GUARD_ZONE: [u32; 16] = [ STACK_GUARD_PATTERN; 16 ];

/// Checks if the stack guard region has been corrupted.
///
/// # Returns
/// `true` if the stack guard is corrupted, `false` otherwise.
pub fn stack_guard_corrupted() -> bool {
    unsafe {
        let stack_guard_ptr = ptr::addr_of!(STACK_GUARD_ZONE);
        for i in 0..(*stack_guard_ptr).len() {
            if (*stack_guard_ptr)[i] != STACK_GUARD_PATTERN {
                return true;
            }
        }
        false
    }
}