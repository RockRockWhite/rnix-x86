[org 0x1000]
dw 0x55aa ; 校验
mov si, loading
call print

detect_memory:
    xor ebx, ebx

    ; es:di 缓冲区地址
    mov ax, 0
    mov es, ax
    mov edi, ards_buffer

    ; 固定签名
    mov edx, 0x534d4150

.next:
    ; 子功能号
    mov eax, 0xe820
    ; ards 大小
    mov ecx, 20
    int 0x15

    jc error ; CF 表示错误

    inc dword [ards_count] ; 计数增加

    add di, cx ;下一个结构体

    ; 判终止
    cmp ebx, 0
    jne .next

    mov si, detecting
    call print

    mov ecx, ards_count
    mov si, 0

.show:
    mov eax, [ards_buffer + si]
    mov ebx, [ards_buffer + si + 8]
    mov edx, [ards_buffer + si + 16]
    add si, 20
    xchg bx, bx
    loop .show
    
    jmp prepare_protected_mode

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

prepare_protected_mode:
    cli ; 关中断

    ; 开A20线
    in al, 0x92;
    or al, 0b10;
    out 0x92, al

    ; 加载GDT
    lgdt [gdt_ptr]

    ; 启动保护模式
    mov eax, cr0
    or eax, 1
    mov cr0, eax

    ; 用跳转刷新段寄存器
    jmp dword code_selector:protected_mode

[bits 32]
protected_mode:
    mov ax, (2 << 3)
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax ; 初始化段寄存器

    mov esp, 0x10000 ; 改栈顶

    ; mov edi, 0x10000 ;读到10000
    ; mov ecx, 10 ; 10扇区开始
    ; mov bl, 200 ; 读200个扇区
    ; call read_disk

    mov edi, 0x10000 ;读到10000
    mov ecx, 10 ; 10扇区开始
    mov ebx, 1000 ; 读1000个扇区

    call read_disk_more

    mov eax, 0x20230531 ; 魔法值
    mov ebx, ards_count ; ards 数量

    jmp dword code_selector:0x10000

    ud2 ; 执行出错

; 读超过256个扇区
; edi 读取内存位置
; ecx 读取开始扇区位置
; ebx 读取扇区数量

read_disk_more: 

.read_once:
    cmp ebx, 200
    jbe .below_or_equal_200

    ; above 200, read 200 each time

    push ebx
    mov ebx, 200

    push ecx
    call read_disk
    pop ecx

    pop ebx
    add ecx, 200
    sub ebx, 200

    jmp .read_once

.below_or_equal_200:
    call read_disk
    ret

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

loading:
    db "Loading System...", 10, 13, 0
detecting:
    db "Detecting Memory succceed...", 10, 13, 0
error: 
    mov si, .msg
    call print
    hlt
    .msg db "Loading error!!!", 10, 13, 0

code_selector equ (1 << 3)
data_selector equ (2 << 3)


memory_base equ 0 ; 基地址
memory_limit equ ((1024 * 1024 * 1024 * 4) / (1024 * 4)) - 1; 4G / 4k(unit) -1 limit
gdt_ptr: ;gdt 指针
    dw (gdt_end - gdt_base) - 1 ; limit
    dd gdt_base
gdt_base:
    dd 0, 0 ; 双字 32位
gdt_code:
    dw memory_limit & 0xffff ; limit取低16位
    dw memory_base & 0xffff ; base取低16位
    db (memory_base >> 16) & 0xff ; base取中8位
    ; 内存 dpl等级0 代码段 非依从 可读 没访问
    db 0b_1_00_1_1010 ; 
    ; 4k 32bit 不扩64bit 
    db 0b_1_1_0_0_0000 | ((memory_limit >> 16) & 0xf);
    db (memory_base >> 24) & 0xff;
gdt_data:
    dw memory_limit & 0xffff ; limit取低16位
    dw memory_base & 0xffff ; base取低16位
    db (memory_base >> 16) & 0xff ; base取中8位
    ; 内存 dpl等级0 数据段 非依从 向上扩展 可写 没访问
    db 0b_1_00_1_0010 ; 
    ; 4k 32bit 不扩64bit 
    db 0b_1_1_0_0_0000 | ((memory_limit >> 16) & 0xf);
    db (memory_base >> 24) & 0xff;
gdt_end:
ards_count:
    dd 0
ards_buffer: