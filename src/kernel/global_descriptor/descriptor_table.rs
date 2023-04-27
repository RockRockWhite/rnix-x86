use super::constants::*;
use modular_bitfield::{bitfield, specifiers::*};

/// 全局描述符表
/// 用于描述内存中的段
/// 一个段描述符占 8 字节
#[bitfield]
#[derive(Copy, Clone)]
pub struct Descriptor {
    pub limit_low: B16,                 // 段界限 0 ~ 15 位
    pub base_low: B24,                  // 基地址 0 ~ 23 位 16M
    pub segment_type: B4,               // 段类型
    pub segment: B1,                    // 1 表示代码段或数据段，0 表示系统段
    pub descriptor_privilege_level: B2, // Descriptor Privilege Level 描述符特权等级 0 ~ 3
    pub present: B1,                    // 存在位，1 在内存中，0 在磁盘上
    pub limit_high: B4,                 // 段界限 16 ~ 19;
    pub available: B1,                  // 该安排的都安排了，送给操作系统吧
    pub long_mode: B1,                  // 64 位扩展标志
    pub big: B1,                        // 32 位 还是 16 位;
    pub granularity: B1,                // 粒度 4KB 或 1B
    pub base_high: B8,                  // 基地址 24 ~ 31 位
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct Selector {
    pub rpl: B2,    // 请求特权级
    pub ti: B1,     // 表指示符，0 表示 GDT，1 表示 LDT
    pub index: B13, // 段描述符索引
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct Pointer {
    pub limit: B16, // 全局描述符表界限, [0, 127] 里面的是bit位数，不是字节数！！！
    pub base: B32,  // 全局描述符表基地址
}

pub struct DescriptorTable {
    pub descriptors: [Descriptor; MAX_GDT],
    pub pointer: Pointer,
}
