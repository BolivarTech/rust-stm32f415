#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ffi;
use cortex_m::{Peripherals, iprintln};

//use panic_halt as _; // Use panic_halt crate for minimal panic handler



extern "C" {
    fn c_main_init();
    fn HAL_Delay(delay: ffi::c_uint);
    //fn HAL_GPIO_TogglePin(GPIO_TypeDef* GPIOx, ffi::c_ushort GPIO_Pin);
}


static mut DELAY_VALUE: ffi::c_uint = 500; // Delay in milliseconds

/// Entry point of the program.
///
/// Runs a loop that calls the recursive test function and checks the stack guard.
/// If the stack guard is corrupted, the loop breaks.
#[no_mangle]
extern "C" fn main() -> ! {
    let mut cp = Peripherals::take().unwrap();
    let stim = &mut cp.ITM.stim[0];

    unsafe {
        c_main_init();
    }


    loop{
        unsafe {
            iprintln!(stim, "Hello from ITM!");

  //          rprintln!("LED toggled"); // not present in --release
            HAL_Delay(DELAY_VALUE); // Delay 500 milliseconds
        }
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

/// Callback for push button events.
///
/// This function is called by the BSP (Board Support Package) when the User button generates an
/// interrupt.
///
/// It toggles the delay value used in the main loop.
///
/// # Arguments
///
/// * `Button` - The ID of the button that triggered the callback.
///
/// Toggles the delay value between 500ms and 1000ms when the user button (ID 0) is pressed.
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn BSP_PB_Callback(Button: ffi::c_uint) {
    unsafe {
        if Button == 0 { // Assuming 0 is the button ID for the user button
            DELAY_VALUE = if DELAY_VALUE == 500 { 1000 } else { 500 }; // Toggle delay between 500ms and 1000ms
        }
    }
}
