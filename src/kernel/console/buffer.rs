#![allow(dead_code)]

use super::constants::*;
use crate::io;
use core::{arch::asm, cmp::Ordering};

pub struct Position {
    x: isize,
    y: isize,
}

pub struct Buffer {
    screen_offset: isize, // 当前屏幕开始字符偏移（离第一个字符偏移多少个字符）
    cursor_offset: isize, // 当前光标开始字符便宜
    cursor_pos: Position, // 当前颜色
    attr: u8,             // 当前颜色
    erase: u16,           // 当前颜色空格
}

impl Buffer {
    pub fn new() -> Self {
        let mut data = Self {
            screen_offset: 0,
            cursor_offset: 0,
            cursor_pos: Position { x: 0, y: 0 },
            attr: 0x07,
            erase: 0x0720,
        };
        data._read_screen_offset();
        data._read_cursor_offset();
        data._update_cursor_pos();
        data
    }

    fn _read_screen_offset(&mut self) {
        // 读高八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::StartAddrHigh as u8);
        self.screen_offset = (io::input_byte(io::Port::CrtDataReg) as isize) << (8 as isize);

        // 读低八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::StartAddrLow as u8);
        self.screen_offset |= io::input_byte(io::Port::CrtDataReg) as isize;
    }

    fn _write_screen_offset(&mut self) {
        // 写高八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::StartAddrHigh as u8);
        io::output_byte(
            io::Port::CrtDataReg,
            (self.screen_offset >> 8 as isize) as u8,
        );

        // 写低八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::StartAddrLow as u8);
        io::output_byte(io::Port::CrtDataReg, self.screen_offset as u8);
    }

    fn _read_cursor_offset(&mut self) {
        // 读高八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::CursorHigh as u8);
        self.cursor_offset = (io::input_byte(io::Port::CrtDataReg) as isize) << (8 as isize);

        // 读低八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::CursorLow as u8);
        self.cursor_offset |= io::input_byte(io::Port::CrtDataReg) as isize;
    }

    fn _update_cursor_pos(&mut self) {
        // 计算光标相对于屏幕的偏移量
        let delta = self.cursor_offset - self.screen_offset;
        self.cursor_pos.x = delta % WIDTH; // 计算光标的x坐标
        self.cursor_pos.y = delta / WIDTH; // 计算光标的y坐标
    }

    fn _write_cursor_offset(&mut self) {
        // 写高八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::CursorHigh as u8);
        io::output_byte(
            io::Port::CrtDataReg,
            (self.cursor_offset >> 8 as isize) as u8,
        );

        // 写低八位
        io::output_byte(io::Port::CrtAddrReg, io::CrtAddr::CursorLow as u8);
        io::output_byte(io::Port::CrtDataReg, (self.cursor_offset as isize) as u8);
    }

    fn _get_curosr_ptr(&self) -> *mut u8 {
        (self.cursor_offset * 2 + MEM_BASE) as *mut u8
    }

    fn _get_screen_ptr(&self) -> *mut u8 {
        (self.screen_offset * 2 + MEM_BASE) as *mut u8
    }

    fn _scrool_up(&mut self) {
        // 如果未超出内存区域，则屏幕下移一行
        // 如果超出了，则拷贝一屏幕到屏幕起始位置
        if self.cursor_offset + WIDTH - self.cursor_pos.x < MEM_SIZE / 2 {
            self.screen_offset += WIDTH;
        } else {
            self.screen_offset += WIDTH;
            let src = self._get_screen_ptr() as *mut u16;
            let dst = MEM_BASE as *mut u16;

            for i in 0..WIDTH * HEIGHT {
                unsafe {
                    *dst.offset(i) = *src.offset(i);
                }
            }

            self.cursor_offset = self.cursor_offset - self.screen_offset;
            self.screen_offset = 0;
        }

        // 下移光标
        self._update_cursor_pos();
        self.cursor_offset += WIDTH - self.cursor_pos.x;

        let ptr = self._get_curosr_ptr() as *mut u16;
        for i in 0..WIDTH {
            unsafe {
                *ptr.offset(i) = self.erase;
            }
        }

        self._write_cursor_offset();
        self._write_screen_offset();
        self._update_cursor_pos();
    }

    pub fn update_change(&mut self) {
        self._write_cursor_offset();
        self._write_screen_offset();
        self._update_cursor_pos();
    }

    pub fn clear(&mut self) {
        self.cursor_offset = 0;
        self.screen_offset = 0;
        self.cursor_pos.x = 0;
        self.cursor_pos.y = 0;

        let ptr = MEM_BASE as *mut u16;

        // 填充空字符串
        for i in 0..SCR_SIZE {
            unsafe {
                *ptr.offset(i) = self.erase;
            }
        }

        self.update_change();
    }

    /// write_byte
    /// 向屏幕写一个字节
    /// 此处通过开关中断保证原子性
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\x07' => {} // \a
            b'\x08' => {
                // 光标回退
                if self.cursor_offset > 0 {
                    self.cursor_offset -= 1;
                }

                // 清理字符
                let cursor_ptr = self._get_curosr_ptr() as *mut u16;
                unsafe {
                    *cursor_ptr = self.erase;
                }
            } // \b 退格
            b'\t' => {}
            b'\n' => {
                match self.cursor_pos.y.cmp(&(HEIGHT - 1)) {
                    Ordering::Less => {
                        // 如果光标不在最后一行, 则光标下移一行
                        self.cursor_offset += WIDTH - self.cursor_pos.x;
                    }
                    _ => {
                        // 如果已经是最后一行，滚屏后再移动光标
                        self._scrool_up();
                    }
                }
            }
            b'\r' => {
                // 光标回退到行首
                self.cursor_offset -= self.cursor_pos.x;
            }
            0x0b => {} // \v
            0x0c => {} // \f
            0x7f => {
                // 删除字符
                let cursor_ptr = self._get_curosr_ptr() as *mut u16;
                unsafe {
                    *cursor_ptr = self.erase;
                }
            } // \del
            _ => {
                let cursor_ptr = self._get_curosr_ptr();
                unsafe {
                    *cursor_ptr = byte;
                    // *cursor_ptr.offset(1) = self.attr;
                }
                self.cursor_offset += 1;

                self._update_cursor_pos();
                // 如果光标超出了屏幕范围，则滚屏
                match self.cursor_pos.y.cmp(&(HEIGHT)) {
                    Ordering::Less => {}
                    _ => {
                        // 如果已经是最后一行，滚屏
                        self._scrool_up();
                        self.cursor_offset -= WIDTH;
                    }
                }
            }
        }
        self.update_change();
    }
}
