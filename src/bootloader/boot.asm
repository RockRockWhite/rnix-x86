[org 0x7c00]

; 设置屏幕为文本模式, 清除屏幕
mov ax, 3
int 0x10

; 初始化段寄存器
mov ax, 0
mov ds, ax
mov es, ax
mov ss, ax
mov sp, 0x7c00

mov si, booting
call print

mov edi, 0x1000 ; 读取的目标内存
mov ecx, 2; 起始扇区
mov bl, 4 ; 扇区数
call read_disk

cmp word[0x1000], 0x55aa
jne error
jmp 0x1000

; 阻塞
jmp $


read_disk:
    mov dx, 0x1f2 ; 读写扇区数量端口
    mov al, bl
    out dx, al

    inc dx ; 0x1f3 低八位
    mov al, cl
    out dx, al

    inc dx ; 0x1f4 中八位
    shr ecx, 8
    mov al, cl
    out dx, al

    inc dx ; 0x1f5 高八位
    shr ecx, 8
    mov al, cl
    out dx, al

    inc dx ; 0x1f6
    shr ecx, 8 ; 0 ~ 3 14 ~ 27 位
    and cl, 0b0000_1111 ; 高位位0
    or al, 0b1110_0000 ; 6 7 位默认为1， 5位逻辑扇区模式
    out dx, al

    inc dx ; 0x1f7
    mov al, 0x20 ; 写硬盘
    out dx, al

    ; 读硬盘
    xor ecx, ecx;
    mov cl, bl ; 计数读写扇区数

    .read:

        call .waits
        call .reads

        loop .read

    ret

    .waits: ; 等待数据准备
        mov dx, 0x1f7;
        .check:
            in al, dx;
            jmp $+2
            jmp $+2
            jmp $+2
            and al, 0b1000_1000 ; 只留下固定位
            cmp al, 0b0000_1000 ; 7位0， 4位1
            jnz .check 
        ret
    
    .reads: ; 读取一个扇区
        push cx

        mov dx, 0x1f0
        mov cx, 256; 一个扇区256个字（16位）
        .readw:
            in ax, dx;
            jmp $+2
            jmp $+2
            jmp $+2
            mov [edi], ax
            add edi, 2
            loop .readw

        pop cx
        ret 

print:
    mov ah, 0x0e

.next:
    mov al, [si]
    cmp al, 0
    je .done

    int 0x10
    inc si
    jmp .next
.done:
    ret

booting:
    db "Booting System...", 10, 13, 0

error: 
    mov si, .msg
    call print
    hlt
    .msg db "Booting error!!!", 10, 13, 0

; 填充mbr
times 510 - ($ - $$) db 0
db 0x55, 0xaa