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

#[no_mangle]
extern "C" fn HardFault_Handler() { loop {} }
#[no_mangle]
extern "C" fn NMI_Handler() { loop {} }

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

    }
    crate::main(); // Call the main function defined in main
}
#[no_mangle]
extern "C" fn Default_Handler() { loop {} }

extern "C" {
    fn BusFault_Handler();
    fn MemManage_Handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn UsageFault_Handler();
    fn ADC1_2_Handler();
    fn AES_Handler();
    fn COMP1_2_3_Handler();
    fn COMP4_5_6_Handler();
    fn COMP7_Handler();
    fn CRS_Handler();
    fn Cordic_Handler();
    fn DMA1_CH1_Handler();
    fn DMA1_CH2_Handler();
    fn DMA1_CH3_Handler();
    fn DMA1_CH4_Handler();
    fn DMA1_CH5_Handler();
    fn DMA1_CH6_Handler();
    fn DMA2_CH1_Handler();
    fn DMA2_CH2_Handler();
    fn DMA2_CH3_Handler();
    fn DMA2_CH4_Handler();
    fn DMA2_CH5_Handler();
    fn DMA2_CH6_Handler();
    fn DMAMUX_OVR_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI1_Handler();
    fn EXTI2_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FDCAN1_IT0_Handler();
    fn FDCAN1_IT1_Handler();
    fn FLASH_Handler();
    fn FMAC_Handler();
    fn FPU_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_Handler();
    fn I2C3_ER_Handler();
    fn I2C3_EV_Handler();
    fn LPTIM1_Handler();
    fn LPUART_Handler();
    fn PVD_PVM_Handler();
    fn RCC_Handler();
    fn RNG_Handler();
    fn RTC_ALARM_Handler();
    fn RTC_TAMP_CSS_LSE_Handler();
    fn RTC_WKUP_Handler();
    fn SAI_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn TIM1_BRK_TIM15_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_Handler();
    fn TIM1_UP_TIM16_Handler();
    fn TIM20_BRK_Handler();
    fn TIM20_CC_Handler();
    fn TIM20_TRG_COM_Handler();
    fn TIM20_UP_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM6_DACUNDER_Handler();
    fn TIM7_Handler();
    fn TIM8_BRK_Handler();
    fn TIM8_CC_Handler();
    fn TIM8_TRG_COM_Handler();
    fn TIM8_UP_Handler();
    fn UART4_Handler();
    fn UCPD1_Handler();
    fn USART1_Handler();
    fn USART2_Handler();
    fn USART3_Handler();
    fn USBWakeUP_Handler();
    fn USB_HP_Handler();
    fn USB_LP_Handler();
    fn WWDG_Handler();
}

#[used]
#[link_section = ".isr_vector"]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 117] = [
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_PVM_Handler),
    Some(RTC_TAMP_CSS_LSE_Handler),
    Some(RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_CH1_Handler),
    Some(DMA1_CH2_Handler),
    Some(DMA1_CH3_Handler),
    Some(DMA1_CH4_Handler),
    Some(DMA1_CH5_Handler),
    Some(DMA1_CH6_Handler),
    None,
    Some(ADC1_2_Handler),
    Some(USB_HP_Handler),
    Some(USB_LP_Handler),
    Some(FDCAN1_IT0_Handler),
    Some(FDCAN1_IT1_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM15_Handler),
    Some(TIM1_UP_TIM16_Handler),
    Some(TIM1_TRG_COM_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_Handler),
    Some(USART2_Handler),
    Some(USART3_Handler),
    Some(EXTI15_10_Handler),
    Some(RTC_ALARM_Handler),
    Some(USBWakeUP_Handler),
    Some(TIM8_BRK_Handler),
    Some(TIM8_UP_Handler),
    Some(TIM8_TRG_COM_Handler),
    Some(TIM8_CC_Handler),
    None,
    None,
    Some(LPTIM1_Handler),
    None,
    Some(SPI3_Handler),
    Some(UART4_Handler),
    None,
    Some(TIM6_DACUNDER_Handler),
    Some(TIM7_Handler),
    Some(DMA2_CH1_Handler),
    Some(DMA2_CH2_Handler),
    Some(DMA2_CH3_Handler),
    Some(DMA2_CH4_Handler),
    Some(DMA2_CH5_Handler),
    None,
    None,
    Some(UCPD1_Handler),
    Some(COMP1_2_3_Handler),
    Some(COMP4_5_6_Handler),
    Some(COMP7_Handler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(CRS_Handler),
    Some(SAI_Handler),
    Some(TIM20_BRK_Handler),
    Some(TIM20_UP_Handler),
    Some(TIM20_TRG_COM_Handler),
    Some(TIM20_CC_Handler),
    Some(FPU_Handler),
    None,
    None,
    None,
    Some(AES_Handler),
    None,
    None,
    None,
    None,
    Some(RNG_Handler),
    Some(LPUART_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(DMAMUX_OVR_Handler),
    None,
    None,
    Some(DMA2_CH6_Handler),
    None,
    None,
    Some(Cordic_Handler),
    Some(FMAC_Handler),
];
