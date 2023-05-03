[bits 32]
extern kernel_init
extern print_handler
global _start
global handler

_start:
    ; xchg bx, bx
    call kernel_init
    ; xchg bx, bx
    ; int 0x80
    
    jmp $