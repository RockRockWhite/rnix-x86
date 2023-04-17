.text
.global inb
inb:
    push ebp
    mov ebp, esp

    xor eax, eax
    mov edx, [ebp + 8] // port
    in al, dx// 将8bit输入到al中

    call wait_a_moment

    leave
    ret

.global outb
outb:
    push ebp
    mov ebp, esp

    mov edx, [ebp + 8] // port
    mov eax, [ebp + 12] // data
    out dx, al// 将8bit输出到al中

    call wait_a_moment

    leave
    ret

.global inw
inw:
    push ebp
    mov ebp, esp

    xor eax, eax
    mov edx, [ebp + 8] // port
    in ax, dx// 将8bit输入到al中

    call wait_a_moment

    leave
    ret

.global outw
outw:
    push ebp
    mov ebp, esp

    mov edx, [ebp + 8] // port
    mov eax, [ebp + 12] // data
    out dx, ax// 将16bit输出到al中

    call wait_a_moment

    leave
    ret

wait_a_moment:
    jmp .L1
.L1:
    jmp .L2
.L2:
    jmp .L3
.L3:
    jmp .L4
.L4:
    ret