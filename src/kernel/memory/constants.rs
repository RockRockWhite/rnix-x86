#![allow(dead_code)]

pub static RNIX_MAGIC: usize = 0x20230531; // 内核魔法数
pub static ZONE_VALID: usize = 1; // 内存区域有效
pub static ZONE_RESERVED: usize = 2; // 内存区域无效
pub static MEMORY_BASE: usize = 0x100000; // 内存基址 大小为1MB
pub static PAGE_PIZE: usize = 0x1000; // 内存大小
