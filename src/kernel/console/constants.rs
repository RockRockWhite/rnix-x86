#![allow(dead_code)]

pub static MEM_BASE: isize = 0xb8000; // 显卡内存起始位置
pub static MEM_SIZE: isize = 0x4000; // 显卡内存大小
pub static MEM_END: isize = MEM_BASE + MEM_SIZE; // 显卡内存结束位置
pub static WIDTH: isize = 80; // 屏幕文本列数
pub static HEIGHT: isize = 25; // 屏幕文本行数
pub static ROW_SIZE: isize = WIDTH * 2; // 显卡内存起始位置
pub static SCR_SIZE: isize = ROW_SIZE * HEIGHT; // 屏幕字节数
