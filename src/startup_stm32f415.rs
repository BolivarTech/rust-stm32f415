use core::ptr;

pub const STACK_GUARD_PATTERN: u32 = 0xCAFECAFE; // Stack Guard pattern

extern "C"  {
    static _sidata: u32; /* Start address for the flash .data section. Defined in linker script */
    static mut _sdata: u32; /* Start address for the RAM .data section. Defined in linker script */
    static mut _edata: u32; /* End address for the RAM .data section. Defined in linker script */
    static mut _sbss: u32; /* Start address for the RAM .bss section. Defined in linker script */
    static mut _ebss: u32; /* End address for the RAM .bss section. Defined in linker script */
    static mut __fill_start__: u32; /* Start address for the fill section. Defined in linker script */
    static mut __fill_end__: u32; /* End address for the fill section. Defined in linker script */
    static mut _stack_guard: u32; /* Begin of the stack guard. Defined in linker script */
    static mut _stack_min: u32; /* End of the stack guard and begin stack. Defined in linker script */
    static mut _start_of_stack: u32; /* Start of the stack. Defined in linker script */
}

extern "C" {
    // These symbols must be defined in your linker script if you use static constructors
    static __preinit_array_start: unsafe extern "C" fn();
    static __preinit_array_end: unsafe extern "C" fn();
    static __init_array_start: unsafe extern "C" fn();
    static __init_array_end: unsafe extern "C" fn();
}

#[inline(always)]
unsafe fn run_constructors(start: *const unsafe extern "C" fn(), end: *const unsafe extern "C" fn()) {
    let mut ctor = start;
    while ctor < end {
        (*ctor)();
        ctor = ctor.add(1);
    }
}

/// The reset handler for the STM32G431 microcontroller.
///
/// This function is called on system reset. It performs the following:
/// - Copies the `.data` section from flash to RAM.
/// - Zeroes the `.bss` section in RAM.
/// - Fills the unused RAM region with zeros (up to `__fill_end__`).
/// - Calls the main application entry point (`crate::main()`).
///
/// # Safety
/// This function performs raw pointer operations and should only be called by the hardware reset vector.
#[no_mangle]
extern "C" fn Reset_Handler() {
    unsafe {

        // Copy the data segment from flash to RAM
        let mut src_flash = ptr::addr_of!(_sidata);
        let mut dst_ram = ptr::addr_of_mut!(_sdata);
        let end = ptr::addr_of_mut!(_edata);
        while dst_ram < end {
            *dst_ram = *src_flash;
            dst_ram =  dst_ram.add(1);
            src_flash = src_flash.add(1);
        }

        // Zero initialize the .bss section
        let mut bss_start = ptr::addr_of_mut!(_sbss);
        let bss_end = ptr::addr_of_mut!(_ebss);
        while bss_start < bss_end {
            *bss_start = 0;
            bss_start = bss_start.add(1);
        }

        // Clear the Stack
        let mut stack_guard = ptr::addr_of_mut!(_stack_guard);
        let mut stack_pointer: *mut u32;
        core::arch::asm!("mrs {}, msp", out(reg) stack_pointer);
        // For safety, we subtract 1 to ensure we don't overwrite the stack data
        stack_pointer = stack_pointer.sub(1);
        while stack_guard < stack_pointer {
            *stack_guard = 0;
            stack_guard = stack_guard.add(1);
        }

        // Initialize the stack guard and stack minimum
        let mut stack_guard = ptr::addr_of_mut!(_stack_guard);
        let stack_min = ptr::addr_of_mut!(_stack_min);
        while stack_guard < stack_min {
            *stack_guard = STACK_GUARD_PATTERN;
            stack_guard = stack_guard.add(1);
        }

        // Fill the unused RAM with a zero value
        let mut fill_start = ptr::addr_of_mut!(__fill_start__);
        let fill_end = ptr::addr_of_mut!(__fill_end__);
        while fill_start < fill_end {
            *fill_start = 0;
            fill_start = fill_start.add(1);
        }

        // Call static constructors (if any)
        run_constructors(
            &__preinit_array_start as *const _,
            &__preinit_array_end as *const _,
        );
        run_constructors(
            &__init_array_start as *const _,
            &__init_array_end as *const _,
        );

    }
    crate::main(); // Call the main function defined in main
}

