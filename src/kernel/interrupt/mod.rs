mod constants;
mod descriptor_table;
pub mod pic;

use crate::sync::UPSafeCell;
use constants::*;
use core::arch::asm;
use descriptor_table::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: UPSafeCell<DescriptorTable> = unsafe { UPSafeCell::new({
        let mut idt = DescriptorTable {
            gates: [Gate::new(); MAX_INT],
            pointer: Pointer::new().with_limit((MAX_INT * 8 - 1 ) as u16),
        };

        // 初始化各中断门
        idt.gates.iter_mut().enumerate().for_each(|(index, gate)| {
            gate.set_selector(1 << 3); // 全部选择代码段
            gate.set_reserved(0);
            gate.set_gate_type(0b1110); // 中断门
            gate.set_segment(0); // 系统段
            gate.set_dpl(0); // 内核态调用
            gate.set_present(1); // 有效
        });

        idt
    }) };
}

pub fn load_idt() {
    let idt = IDT.borrow();
    // 更改中断描述符表
    unsafe {
        asm!("lidt [{}]", in(reg) &idt.pointer as *const Pointer);
    }
}

pub fn set_handler(index: usize, handler: usize) {
    if index >= MAX_INT {
        panic!("set_handler() failed: index out of range");
    }

    let base_ptr: *const Gate = IDT.borrow().gates.as_ptr();
    let mut idt = IDT.borrow_mut();
    idt.pointer.set_base(base_ptr as *const Gate as u32);

    idt.gates[index].set_offset0(handler as u16);
    idt.gates[index].set_offset1((handler >> 16) as u16);
}
