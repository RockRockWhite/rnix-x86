mod constants;
mod descriptor_table;

use crate::sync::UPSafeCell;
use constants::*;
use core::arch::asm;
use descriptor_table::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref GDT: UPSafeCell<DescriptorTable> = unsafe {
        UPSafeCell::new({
            let mut gdt = DescriptorTable {
                descriptors: [Descriptor::new(); MAX_GDT],
                pointer: Pointer::new().with_limit((MAX_GDT * 8 - 1) as u16),
            };

            // 读取旧的gdt胖指针
            let ptr: *mut Pointer;
            asm!("sgdt [{}]", out(reg) ptr);
            let ptr = *ptr;

            let limit = ptr.limit() as usize / 8;
            let src = ptr.base() as *const Descriptor;

            for i in 1..limit + 1 {
                gdt.descriptors[i] = *src.add(i);
            }

            gdt
        })
    };
}

pub fn load_gdt() {
    let base_ptr: *const Descriptor = GDT.borrow().descriptors.as_ptr();
    let mut gdt = GDT.borrow_mut();
    gdt.pointer.set_base(base_ptr as *const Descriptor as u32);

    // 更改中断描述符表
    unsafe {
        asm!("lgdt [{}]", in(reg) &gdt.pointer as *const Pointer);
    }
}
