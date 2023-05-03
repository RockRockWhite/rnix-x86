use core::arch::{asm, global_asm};

mod constants;
mod pcb;

global_asm!(include_str!("switch.s"));

pub use pcb::Task;

use crate::{interrupt, print};

#[no_mangle]
pub extern "C" fn schedule() {
    let curr = Task::running_task();

    let next = if curr == 0x1000 as *const Task {
        0x2000 as *const Task
    } else {
        0x1000 as *const Task
    };

    interrupt::pic::send_eoi(0x20);

    print!("-S-");
    unsafe {
        next.as_ref().unwrap().switch();
    }
}
