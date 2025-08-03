#![no_std]
#![no_main]

// Import necessary crates for embedded development
use panic_halt as _; // Panic handler for no_std environment
use cortex_m_rt::entry; // Runtime entry point

// Import STM32F4xx HAL for device-specific support including interrupt vectors
use stm32f4xx_hal as hal;
use hal::{pac, prelude::*, gpio::{Output, OpenDrain, Pin}};

#[entry]
fn main() -> ! {
    // Get access to the device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the system clock to run at 168 MHz from HSE
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(25.MHz()) // External crystal is 25 MHz
        .sysclk(168.MHz())  // System clock at 168 MHz
        .freeze();

    // Create a delay provider using SysTick
    let mut delay = cp.SYST.delay(&clocks);

    // Configure GPIO PA8 as open collector output
    let gpioa = dp.GPIOA.split();
    let mut pa8: Pin<'A', 8, Output<OpenDrain>> = gpioa.pa8.into_open_drain_output();

    // Your STM32F415 initialization code will go here
    
    loop {
        // Toggle PA8 (set high - open drain will make it high-impedance)
        pa8.set_high();
        delay.delay_ms(1000_u32); // Wait 1 second
        
        // Toggle PA8 (set low - open drain will pull it to ground)
        pa8.set_low();
        delay.delay_ms(1000_u32); // Wait 1 second
    }
}
