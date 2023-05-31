#![allow(dead_code)]

use modular_bitfield::{bitfield, specifiers::*};

#[bitfield]
#[derive(Copy, Clone)]
pub struct AddressRangeDescriptor {
    pub base_address: B64,
    pub length: B64,
    pub region_type: B32,
}
