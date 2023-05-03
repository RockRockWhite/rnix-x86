#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod global_descriptor;
mod interrupt;
mod io;
mod lang_items;
mod sync;
mod task;

use core::arch::asm;
use task::Task;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    global_descriptor::load_gdt();

    let task_a = unsafe { Task::from_ptr(0x1000, a_func).as_ref().unwrap() };
    let task_b = unsafe { Task::from_ptr(0x2000, b_func).as_ref().unwrap() };

    interrupt::pic::init();
    extern "C" {
        pub fn handler();
        pub fn time_interrupt_handler();
    }
    interrupt::set_handler(0x80, handler as usize);
    interrupt::load_idt();
    interrupt::set_handler(0x20, time_interrupt_handler as usize);

    unsafe {
        asm!("sti");
    }

    loop {}
}

fn a_func() {
    loop {
        unsafe {
            asm!("cli");
        }

        for _ in 0..1000000 {
            unsafe {
                asm!("nop");
            }
        }

        print!("A");
        unsafe {
            asm!("sti");
        }
    }
}

fn b_func() {
    loop {
        unsafe {
            asm!("cli");
        }

        interrupt::pic::send_eoi(0x20);

        for _ in 0..1000000 {
            unsafe {
                asm!("nop");
            }
        }

        print!("B");
        unsafe {
            asm!("sti");
        }
    }
}

#[no_mangle]
pub extern "C" fn print_handler() {
    println!("i 'm in interrupt handler")
}
