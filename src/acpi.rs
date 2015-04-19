use core::prelude::*;
use core::mem::size_of;
use collections::Vec;
use alloc::boxed::Box;

use paging;
use interrupts::apic::num_interrupts;

#[repr(packed)]
struct RSDP {
    signature: [u8; 8],
    _checksum: u8,
    _oemid: [u8; 6],
    _revision: u8,
    rsdt_address: u32,
}

#[repr(packed)]
pub struct Header {
    signature: [u8; 4],
    length: u32,
    _revision: u8,
    _checksum: u8,
    _oemid: [u8; 6],
    _oem_table_id: [u8; 8],
    _oem_revision: u32,
    _creatorid: u32,
    _creator_revision: u32,
}

#[repr(packed)]
struct RSDT {
    header: Header,
    _table_ptr: u32,
}

static mut RSDT_CACHE: Option<&'static RSDT> = None;

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

fn find_rsdp() -> Option<&'static RSDP> {
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

unsafe fn load_rsdt() -> &'static RSDT {
    match RSDT_CACHE {
        Some(x) => x,
        None => {
            let rsdp = find_rsdp().unwrap();
            paging::identity_map(rsdp.rsdt_address as usize);
            let maybe_rsdt = rsdp.rsdt_address as *const RSDT;
            if checksum(maybe_rsdt as *const u8, (*maybe_rsdt).header.length as isize) {
                RSDT_CACHE = Some(&*maybe_rsdt);
            }
            &*maybe_rsdt
        }
    }
}

pub fn find_table(name: &[u8; 4]) -> Option<&'static Header> {
    unsafe {
        let rsdt = load_rsdt();
        let rsdt_start = rsdt as *const RSDT as usize;
        let mut addr = rsdt_start + size_of::<Header>();
        let stop = rsdt_start + (rsdt.header.length as usize);
        while addr < stop {
            let header_loc = *(addr as *const u32);
            paging::identity_map(header_loc as usize);
            let header = header_loc as *const Header;
            if (*header).signature == *name {
                return Some(&*header);
            }
            addr += 4;
        }
        None
    }
}

#[repr(packed)]
struct MADTEntry {
    kind: u8,
    length: u8,
    f0: u8,
    f1: u8,
    f2: u32,
    f3: u32,
}

pub struct IOAPIC {
    pub addr: usize,
    pub id: u8,
    pub first_interrupt: u32,
    pub last_interrupt: u32,
}

pub struct SMPInfo {
    pub processors: Vec<u8>,
    pub io_apics: Vec<IOAPIC>
}

impl SMPInfo {
    fn new() -> SMPInfo {
        SMPInfo { processors: Vec::with_capacity(4),
                  io_apics: Vec::with_capacity(2),
        }
    }
}

pub fn smp_info() -> Box<SMPInfo> {
    let mut ret = SMPInfo::new();
    let table = find_table(b"APIC").unwrap();
    let mut addr = table as *const Header as usize;
    let stop = table.length as usize + addr;
    addr += size_of::<Header>() + 0x8;
    while addr < stop {
        let entry = unsafe { &*(addr as *const MADTEntry) };
        addr += entry.length as usize;
        match entry.kind {
            0 => {
                if entry.f2 & 1 == 1 {
                    ret.processors.push(entry.f1);
                }
            },
            1 => {
                let id = entry.f0;
                let addr = entry.f2 as usize;
                paging::identity_map(addr);
                let first_interrupt = entry.f3;
                let last_interrupt = first_interrupt + num_interrupts(addr);
                ret.io_apics.push(
                    IOAPIC { id: id, addr: addr,
                             first_interrupt: first_interrupt,
                             last_interrupt: last_interrupt,
                    });
            },
            _ => (),
        }
    }
    Box::new(ret)
}
