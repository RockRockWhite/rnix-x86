mod address_range_descriptor;
mod constants;
mod memory_state;

use self::{address_range_descriptor::AddressRangeDescriptor, memory_state::MemoryState};
use crate::{console::print, println, sync::UPSafeCell};
use constants::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref state: UPSafeCell<MemoryState> = unsafe {
        UPSafeCell::new({
            let mut res = MemoryState {
                memory_base: 0,
                memory_size: 0,
                total_pages: 0,
                free_pages: 0,
            };

            res
        })
    };
}

fn index(addr: usize) -> usize {
    addr >> 12
}

pub fn memory_init(magic: usize, addr: usize) {
    if magic != RNIX_MAGIC {
        panic!("Invalid magic number: 0x{:x}", magic);
    }

    let count = unsafe { *(addr as *const usize) };
    let ptr = (addr + 4) as *const AddressRangeDescriptor;

    for i in 0..count {
        let ards = unsafe { ptr.offset(i as isize).as_ref().unwrap() };
        // 十六进制输出 base
        println!(
            "Base: {:x}, Length: {:x}, RegionType: {:x}",
            ards.base_address(),
            ards.length(),
            ards.region_type()
        );

        // 选出最大的内存区域
        if ards.region_type() as usize == ZONE_VALID
            && ards.length() as usize > state.borrow().memory_size
        {
            state.borrow_mut().memory_base = ards.base_address() as usize;
            state.borrow_mut().memory_size = ards.length() as usize;
        }
    }

    // 必须是1M开始
    assert_eq!(state.borrow().memory_base as usize, MEMORY_BASE);
    // 必须是4K对齐
    assert_eq!(state.borrow().memory_size as usize & 0xfff, 0);

    let total_pages = index(state.borrow().memory_size) + index(MEMORY_BASE);
    let free_pages = index(state.borrow().memory_size);

    state.borrow_mut().total_pages = total_pages;
    state.borrow_mut().free_pages = free_pages;

    println!(
        "memory_base: 0x{:x}, memory_size: 0x{:x}, total_pages: {}, free_pages: {}",
        state.borrow().memory_base,
        state.borrow().memory_size,
        state.borrow().total_pages,
        state.borrow().free_pages
    );
}
