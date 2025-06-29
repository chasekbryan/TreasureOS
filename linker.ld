ENTRY(_start);
OUTPUT_FORMAT("elf32-i386");
OUTPUT_ARCH(i386);

SECTIONS
{
    /* Multiboot header at file offset 0 */
    . = 0;
    .multiboot ALIGN(4) :
    {
        KEEP(*(.multiboot))
    }

    /* Kernel at 1 MiB in memory */
    . = 0x00100000;
    .text :
    {
        *(.text*)
    }

    .rodata : { *(.rodata*) }
    .data   : { *(.data*) }
    .bss    : { *(.bss*) *(COMMON) }
}
