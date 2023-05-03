[bits 32]
extern print_handler
global handler
handler:
    call print_handler
    iret

extern schedule
global time_interrupt_handler
time_interrupt_handler:
    call schedule
    iret