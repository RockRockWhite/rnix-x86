use core::arch::asm;

use super::constants::PAGE_SIZE;

/// 栈帧信息
#[repr(C)]
pub struct TaskFrame {
    pub edi: usize,
    pub esi: usize,
    pub ebx: usize,
    pub ebp: usize,
    pub eip: usize,
}

/// 任务的进程控制块
#[repr(C)]
pub struct Task {
    pub stack: *mut TaskFrame, // 任务栈信息
}

impl Task {
    pub unsafe fn from_ptr(ptr: usize, target: fn()) -> *mut Task {
        // 得到栈底地址
        let stack_bottom = ptr + PAGE_SIZE;
        // 给TaskFrame分配空间
        let task_frame = (stack_bottom - core::mem::size_of::<TaskFrame>()) as *mut TaskFrame;

        // 保存callee-saved寄存器
        (*task_frame).edi = 0x11111111;
        (*task_frame).esi = 0x22222222;
        (*task_frame).ebx = 0x33333333;
        (*task_frame).ebp = 0x44444444;

        // 保存eip, 即要切换时执行的指令的地址
        (*task_frame).eip = target as usize;

        // 保存任务栈地址
        let ptr = ptr as *mut Task;
        (*ptr).stack = task_frame;
        ptr
    }

    // 获取当前运行的任务
    fn running_task() -> *const Task {
        unsafe {
            let mut x: usize;
            asm!("mov {}, esp", out(reg) x);

            // 取到一页开始的位置
            x &= 0xfffff000;
            x as *const Task
        }
    }

    // 切换到该任务
    pub fn switch(&self) {
        extern "C" {
            fn task_switch(next: *const Task);
        }
        unsafe { task_switch(self as *const Task) }
    }
}
