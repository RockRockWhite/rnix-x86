use core::arch::asm;
use modular_bitfield::{bitfield, specifiers::*};

/// 全局描述符表
/// 用于描述内存中的段
/// 一个段描述符占 8 字节
#[bitfield]
#[derive(Copy, Clone)]
struct Descriptor {
    limit_low: B16,                 // 段界限 0 ~ 15 位
    base_low: B24,                  // 基地址 0 ~ 23 位 16M
    segment_type: B4,               // 段类型
    segment: B1,                    // 1 表示代码段或数据段，0 表示系统段
    descriptor_privilege_level: B2, // Descriptor Privilege Level 描述符特权等级 0 ~ 3
    present: B1,                    // 存在位，1 在内存中，0 在磁盘上
    limit_high: B4,                 // 段界限 16 ~ 19;
    available: B1,                  // 该安排的都安排了，送给操作系统吧
    long_mode: B1,                  // 64 位扩展标志
    big: B1,                        // 32 位 还是 16 位;
    granularity: B1,                // 粒度 4KB 或 1B
    base_high: B8,                  // 基地址 24 ~ 31 位
}

#[bitfield]
#[derive(Copy, Clone)]
struct Selector {
    rpl: B2,    // 请求特权级
    ti: B1,     // 表指示符，0 表示 GDT，1 表示 LDT
    index: B13, // 段描述符索引
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct Pointer {
    limit: B16, // 全局描述符表界限, [0, 127] 里面的是bit位数，不是字节数！！！
    base: B32,  // 全局描述符表基地址
}

pub struct Gdt {
    descriptors: [Descriptor; 128],
    pointer: Pointer,
}

impl Gdt {
    pub fn new() -> Self {
        let mut gdt = Gdt {
            descriptors: [Descriptor::new(); 128],
            pointer: Pointer::new().with_limit(128 * 8 - 1),
        };

        gdt.pointer
            .set_base(&gdt.descriptors as *const Descriptor as u32);

        let x: u32;
        unsafe {
            asm!("mov {}, 5", out(reg) x);
        }
        let res = x;

        // store gdt
        let mut src = Pointer::new();
        let mut ptr = &mut src as *mut Pointer;
        unsafe {
            asm!("sgdt [{}]", out(reg) ptr);
        }

        unsafe {
            let limit = src.limit() as usize / 8;
            let src = src.base() as *const Descriptor;

            for i in 1..limit + 1 {
                gdt.descriptors[i] = *src.add(i);
            }
        }

        // load gdt
        unsafe {
            asm!("lgdt [{}]", in(reg) &gdt.pointer as *const Pointer);
        }

        gdt
    }
}
