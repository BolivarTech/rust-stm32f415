/* Memory layout for STM32F415RG */
/* STM32F415RG has 1MB Flash and 192KB RAM (128KB + 64KB CCM) */

/* Specify the memory areas */
MEMORY
{
  /* Main RAM - 128KB */
  RAM (xrw)      : ORIGIN = 0x20000000, LENGTH = 128K
  /* Core Coupled Memory (CCM) - 64KB - faster access, no DMA */
  CCMRAM (xrw)   : ORIGIN = 0x10000000, LENGTH = 64K  
  /* Flash memory - 1MB */
  FLASH (rx)     : ORIGIN = 0x08000000, LENGTH = 1024K
}

/* Stack configuration */
/* Place stack at the end of main RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Optional: Minimum stack and heap sizes for verification */
_Min_Heap_Size = 0x200;      /* 512 bytes minimum heap */
_Min_Stack_Size = 0x400;     /* 1KB minimum stack */
