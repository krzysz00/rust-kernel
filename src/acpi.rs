use core::prelude::*;
use core::mem::size_of;

#[repr(C,packed)]
pub struct RSDP {
    signature: [u8; 8],
    _checksum: u8,
    _oemid: [u8; 6],
    _revision: u8,
    rsdt_address: u32,
}

unsafe fn checksum(start: *const u8, length: isize) -> bool {
    let mut sum = 0u32;
    for i in 0..length {
        sum += *start.offset(i) as u32;
    }
    (sum & 0xff) == 0
}

fn validate_rsdp(candidate: &RSDP) -> bool {
    let sig: &[u8] = &(*candidate).signature;
    if sig != b"RSD PTR " {
        return false
    }
    else {
        unsafe {
            checksum(candidate as *const RSDP as *const u8, size_of::<RSDP>() as isize)
        }
    }
}

pub fn find_rsdp() -> Option<&'static RSDP> {
    let high_mem_iter = (0xE0_000..0x100_000).step_by(0x10);
    let ebda_address =
        unsafe {
            *(0x40E as *const u16) as usize * 0x10
        };
    let ebda_iter = (ebda_address..ebda_address + 1024).step_by(0x10);
    for addr in high_mem_iter.chain(ebda_iter) {
        let candidate = unsafe {
            &*(addr as *const RSDP)
        };
        if validate_rsdp(candidate) {
            return Some(candidate)
        }
    }
    None
}
