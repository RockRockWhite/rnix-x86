use core::fmt::{self, Write};

use crate::io;
use crate::sync::UPSafeCell;
use buffer::*;

use lazy_static::lazy_static;
mod buffer;
mod constants;

lazy_static! {
    static ref BUFFER: UPSafeCell<Buffer> = unsafe { UPSafeCell::new(Buffer::new()) };
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &c in s.as_bytes() {
            BUFFER.borrow_mut().write_byte_mem(c);
        }
        BUFFER.borrow_mut().update_change();

        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

// pub struct Console;

// impl Console {
//     pub fn write_byte(c: u8) {
//         BUFFER.borrow_mut().write_byte_mem(c);
//         BUFFER.borrow_mut().update_change();
//     }

//     pub fn write_str(s: &str) {
//         for &c in s.as_bytes() {
//             BUFFER.borrow_mut().write_byte_mem(c);
//         }
//         BUFFER.borrow_mut().update_change();
//     }

//     pub fn write_str_line(s: &str) {
//         Self::write_str(s);
//         Self::write_str("\n");
//     }

//     pub fn write_u32(num: u32) {
//         // 输出0
//         if num == 0 {
//             BUFFER.borrow_mut().write_byte_mem(b'0');
//             BUFFER.borrow_mut().update_change();

//             return;
//         }

//         // 输出数字
//         let mut buffer: [u8; 10] = [0; 10];

//         let mut it = buffer.iter_mut();

//         let mut num = num;

//         while num != 0 {
//             *it.next().unwrap() = (num % 10) as u8 + b'0';
//             num /= 10;
//         }

//         buffer.iter().rev().for_each(|&c| {
//             if c == 0 {
//                 return;
//             }
//             BUFFER.borrow_mut().write_byte_mem(c);
//         });

//         BUFFER.borrow_mut().update_change()
//     }

//     pub fn write_u32_line(num: u32) {
//         Self::write_u32(num);
//         Self::write_str("\n");
//     }

//     pub fn write_i32(num: i32) {
//         // 输出负数
//         if num < 0 {
//             Self::write_byte(b'-');
//         }

//         // 输出数字
//         Self::write_u32(num.abs_diff(0));
//     }

//     pub fn write_i32_line(num: i32) {
//         Self::write_i32(num);
//         Self::write_str("\n");
//     }
// }
