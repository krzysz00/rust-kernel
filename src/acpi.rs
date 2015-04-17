use core::mem::size_of;

#[repr(C,packed)]
pub struct RSDP {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
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

// pub fn find_RSDP() -> &'static RSDP {
//     unsafe {

//     }
// }
