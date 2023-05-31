[bits 32]
extern print_handler
global handler
handler:
    call print_handler
    iret

extern schedule
global timer_interrupt_handler
timer_interrupt_handler:
    push ds
    push es
    push fs
    pusha

    call schedule

    popa
    pop fs
    pop es
    pop ds

    iret