#![allow(dead_code)]

use super::constants::*;
use crate::io;

/// 初始化中断控制器
pub fn init() {
    io::output_byte(io::Port::PicMasterCmd, 0b00010001); // ICW1：边沿触发，级联-8259，需要ICW4
    io::output_byte(io::Port::PicMasterData, 0x20); // ICW2: 起始端口号 0x20
    io::output_byte(io::Port::PicMasterData, 0b00000001); // ICW3: IR2接从片
    io::output_byte(io::Port::PicMasterData, 0b00000001); // ICW4: 8086模式，正常EOI

    io::output_byte(io::Port::PicSlaveCmd, 0b00010001); // ICW1：边沿触发，级联-8259，需要ICW4
    io::output_byte(io::Port::PicSlaveData, 0x28); // ICW2: 起始端口号 0x28
    io::output_byte(io::Port::PicSlaveData, 0b00000010); // ICW3: 设置从片连接到主片的IR2引脚
    io::output_byte(io::Port::PicSlaveData, 0b00000001); // ICW4: 8086模式，正常EOI

    io::output_byte(io::Port::PicMasterData, 0b11111110); // 关闭所有中断
    io::output_byte(io::Port::PicSlaveData, 0b11111111); // 关闭所有中断
}

/// 发送中断结束信号
pub fn send_eoi(vector: usize) {
    if vector >= 0x20 && vector < 0x28 {
        io::output_word(io::Port::PicMasterCmd, PIC_EOI);
    }
    if vector >= 0x28 {
        io::output_word(io::Port::PicMasterCmd, PIC_EOI);
        io::output_word(io::Port::PicSlaveCmd, PIC_EOI);
    }
}
