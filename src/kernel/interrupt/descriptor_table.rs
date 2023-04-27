use super::constants::*;
use modular_bitfield::{bitfield, specifiers::*};

#[bitfield]
#[derive(Copy, Clone)]
pub struct Pointer {
    pub limit: B16, // 界限, [0, 127] 里面的是bit位数，不是字节数！！！
    pub base: B32,  // 基地址
}

#[bitfield]
#[derive(Copy, Clone)]
pub struct Gate {
    pub offset0: B16,  // 段内偏移 0 ~ 15 位
    pub selector: B16, // 代码段选择子
    pub reserved: B8,  // 保留不用
    pub gate_type: B4, // 任务门/中断门/陷阱门
    pub segment: B1,   // 0 表示系统段
    pub dpl: B2,       // 使用int指令访问的最低权限
    pub present: B1,   // 是否有效
    pub offset1: B16,  // 段内偏移：16 ~ 31 位
}

// #[repr(C)]
pub struct DescriptorTable {
    pub gates: [Gate; MAX_INT],
    pub pointer: Pointer,
}
