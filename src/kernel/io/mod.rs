#![allow(dead_code)]

use core::arch::global_asm;

global_asm!(include_str!("io.s"));

extern "C" {
    fn inb(port: u16) -> u8;
    fn inw(port: u16) -> u16;

    fn outb(port: u16, data: u8);
    fn outw(port: u16, data: u16);
}

pub enum Port {
    CrtAddrReg = 0x3d4,   // CRT(6845)索引寄存器
    CrtDataReg = 0x3d5,   // CRT(6845)数据寄存器
    PicMasterCmd = 0x20,  // 主片的控制端口
    PicMasterData = 0x21, // 主片的数接端口
    PicSlaveCmd = 0xa0,   // 从片的控制端口
    PicSlaveData = 0xa1,  // 从片的数据端口
}

pub enum CrtAddr {
    CursorHigh = 0xe,    // 光标位置 - 高位
    CursorLow = 0xf,     // 光标位置 - 低位
    StartAddrHigh = 0xc, // 显示内存起始位置 - 高位
    StartAddrLow = 0xd,  // 显示内存起始位置 - 低位
}

pub fn input_byte(port: Port) -> u8 {
    unsafe { inb(port as u16) }
}

pub fn input_word(port: Port) -> u16 {
    unsafe { inw(port as u16) }
}

pub fn output_byte(port: Port, data: u8) {
    unsafe { outb(port as u16, data) }
}

pub fn output_word(port: Port, data: u16) {
    unsafe { outw(port as u16, data) }
}
