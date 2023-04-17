.global task_switch

task_switch:
    push ebp
    mov ebp, esp

    push ebx
    push esi
    push edi

    mov eax, esp
    and eax, 0xFFFFF000  // Task变量的地址

    mov [eax], esp // 保存当前任务栈信息

    mov eax, [ebp + 8] // 读next参数
    mov esp, [eax] // 切换任务栈

    pop edi
    pop esi
    pop ebx
    pop ebp

    ret