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
extern "C" fn HardFault_Handler() { loop {} }
#[no_mangle]
extern "C" fn NMI_Handler() { loop {} }
#[no_mangle]
extern "C" fn Default_Handler() { loop {} }

extern "C" {
    fn BusFault_Handler();
    fn MemManage_Handler();
    fn PendSV_Handler();
    fn SVCall_Handler();
    fn SysTick_Handler();
    fn UsageFault_Handler();
    fn ADC_Handler();
    fn CAN1_RX0_Handler();
    fn CAN1_RX1_Handler();
    fn CAN1_SCE_Handler();
    fn CAN1_TX_Handler();
    fn CAN2_RX0_Handler();
    fn CAN2_RX1_Handler();
    fn CAN2_SCE_Handler();
    fn CAN2_TX_Handler();
    fn CRYP_Handler();
    fn DCMI_Handler();
    fn DMA1_Stream0_Handler();
    fn DMA1_Stream1_Handler();
    fn DMA1_Stream2_Handler();
    fn DMA1_Stream3_Handler();
    fn DMA1_Stream4_Handler();
    fn DMA1_Stream5_Handler();
    fn DMA1_Stream6_Handler();
    fn DMA1_Stream7_Handler();
    fn DMA2_Stream0_Handler();
    fn DMA2_Stream1_Handler();
    fn DMA2_Stream2_Handler();
    fn DMA2_Stream3_Handler();
    fn DMA2_Stream4_Handler();
    fn DMA2_Stream5_Handler();
    fn DMA2_Stream6_Handler();
    fn DMA2_Stream7_Handler();
    fn ETH_Handler();
    fn ETH_WKUP_Handler();
    fn EXTI0_Handler();
    fn EXTI15_10_Handler();
    fn EXTI1_Handler();
    fn EXTI2_Handler();
    fn EXTI3_Handler();
    fn EXTI4_Handler();
    fn EXTI9_5_Handler();
    fn FLASH_Handler();
    fn FPU_Handler();
    fn FSMC_Handler();
    fn HASH_RNG_Handler();
    fn I2C1_ER_Handler();
    fn I2C1_EV_Handler();
    fn I2C2_ER_Handler();
    fn I2C2_EV_Handler();
    fn I2C3_ER_Handler();
    fn I2C3_EV_Handler();
    fn OTG_FS_Handler();
    fn OTG_FS_WKUP_Handler();
    fn OTG_HS_EP1_IN_Handler();
    fn OTG_HS_EP1_OUT_Handler();
    fn OTG_HS_Handler();
    fn OTG_HS_WKUP_Handler();
    fn PVD_Handler();
    fn RCC_Handler();
    fn RTC_Alarm_Handler();
    fn RTC_WKUP_Handler();
    fn SDIO_Handler();
    fn SPI1_Handler();
    fn SPI2_Handler();
    fn SPI3_Handler();
    fn TAMP_STAMP_Handler();
    fn TIM1_BRK_TIM9_Handler();
    fn TIM1_CC_Handler();
    fn TIM1_TRG_COM_TIM11_Handler();
    fn TIM1_UP_TIM10_Handler();
    fn TIM2_Handler();
    fn TIM3_Handler();
    fn TIM4_Handler();
    fn TIM5_Handler();
    fn TIM6_DAC_Handler();
    fn TIM7_Handler();
    fn TIM8_BRK_TIM12_Handler();
    fn TIM8_CC_Handler();
    fn TIM8_TRG_COM_TIM14_Handler();
    fn TIM8_UP_TIM13_Handler();
    fn UART4_Handler();
    fn UART5_Handler();
    fn USART1_Handler();
    fn USART2_Handler();
    fn USART3_Handler();
    fn USART6_Handler();
    fn WWDG_Handler();
}

#[used]
#[link_section = ".isr_vector"]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 97] = [
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
    Some(PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_Stream0_Handler),
    Some(DMA1_Stream1_Handler),
    Some(DMA1_Stream2_Handler),
    Some(DMA1_Stream3_Handler),
    Some(DMA1_Stream4_Handler),
    Some(DMA1_Stream5_Handler),
    Some(DMA1_Stream6_Handler),
    Some(ADC_Handler),
    Some(CAN1_TX_Handler),
    Some(CAN1_RX0_Handler),
    Some(CAN1_RX1_Handler),
    Some(CAN1_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM9_Handler),
    Some(TIM1_UP_TIM10_Handler),
    Some(TIM1_TRG_COM_TIM11_Handler),
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
    Some(RTC_Alarm_Handler),
    Some(OTG_FS_WKUP_Handler),
    Some(TIM8_BRK_TIM12_Handler),
    Some(TIM8_UP_TIM13_Handler),
    Some(TIM8_TRG_COM_TIM14_Handler),
    Some(TIM8_CC_Handler),
    Some(DMA1_Stream7_Handler),
    Some(FSMC_Handler),
    Some(SDIO_Handler),
    Some(TIM5_Handler),
    Some(SPI3_Handler),
    Some(UART4_Handler),
    Some(UART5_Handler),
    Some(TIM6_DAC_Handler),
    Some(TIM7_Handler),
    Some(DMA2_Stream0_Handler),
    Some(DMA2_Stream1_Handler),
    Some(DMA2_Stream2_Handler),
    Some(DMA2_Stream3_Handler),
    Some(DMA2_Stream4_Handler),
    Some(ETH_Handler),
    Some(ETH_WKUP_Handler),
    Some(CAN2_TX_Handler),
    Some(CAN2_RX0_Handler),
    Some(CAN2_RX1_Handler),
    Some(CAN2_SCE_Handler),
    Some(OTG_FS_Handler),
    Some(DMA2_Stream5_Handler),
    Some(DMA2_Stream6_Handler),
    Some(DMA2_Stream7_Handler),
    Some(USART6_Handler),
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(OTG_HS_EP1_OUT_Handler),
    Some(OTG_HS_EP1_IN_Handler),
    Some(OTG_HS_WKUP_Handler),
    Some(OTG_HS_Handler),
    Some(DCMI_Handler),
    Some(CRYP_Handler),
    Some(HASH_RNG_Handler),
    Some(FPU_Handler),
];
