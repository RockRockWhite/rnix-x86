[bits 32]
extern kernel_init
global _start

_start:
    xchg bx, bx
    call kernel_init
    xchg bx, bx

    ; 调用中断函数
    mov eax, 0
    mov edx, 0
    div edx
    
    jmp $