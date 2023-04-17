#![no_std]
#![no_main]
#![feature(panic_info_message)]

use global_descriptor::Gdt;
use task::Task;

mod console;
mod global_descriptor;
mod io;
mod lang_items;
mod sync;
mod task;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    let gdt = Gdt::new();

    let task_a = unsafe { Task::from_ptr(0x1000, a_func).as_ref().unwrap() };

    let task_b = unsafe { Task::from_ptr(0x2000, b_func).as_ref().unwrap() };

    task_a.switch();

    loop {}
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
