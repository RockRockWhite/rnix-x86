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
pub extern "C" fn kernel_init() {
    global_descriptor::load_gdt();

    let task_a = unsafe { Task::from_ptr(0x1000, a_func).as_ref().unwrap() };
    let task_b = unsafe { Task::from_ptr(0x2000, b_func).as_ref().unwrap() };

    extern "C" {
        fn handler();
    }

    interrupt::set_handler(0x80, handler as usize);
    interrupt::load_idt();

    // let res = ;

    // println!("res:{}", res);åå
    // task_a.switch();

    unsafe {
        asm!("int 0x80");
    }

    // loop {}
}

fn a_func() {
    let mut res = 0;
    loop {
        println!("a res:{}", res);
        res += 1;

        unsafe {
            let task = 0x2000 as *mut Task;
            task.as_ref().unwrap().switch();
        }
    }
}

fn b_func() {
    let mut res = 0;
    loop {
        println!("b res:{}", res);
        res += 1;

        unsafe {
            let task_a = 0x1000 as *mut Task;
            task_a.as_ref().unwrap().switch();
        }
    }
}

#[no_mangle]
pub extern "C" fn print_handler() {
    println!("i 'm in interrupt handler")
}
