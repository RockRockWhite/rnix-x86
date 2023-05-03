use core::{
    arch::asm,
    fmt::{self, Write},
};

use crate::sync::UPSafeCell;
use buffer::*;
use lazy_static::lazy_static;

mod buffer;
mod constants;

lazy_static! {
    static ref BUFFER: UPSafeCell<Buffer> = unsafe { UPSafeCell::new(Buffer::new()) };
}

struct Stdout;

/// 多个核心使用的时候，会造成panic！
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &c in s.as_bytes() {
            BUFFER.borrow_mut().write_byte(c);
        }
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
