use core::prelude::*;
use machine::{outb, inb, inl};
use alloc::heap::allocate;
use core::slice;
use collections::vec::Vec;

const SECTOR_BYTES: usize = 512;
const PORTS: [u16; 2] = [0x1f0, 0x170];

#[inline]
fn controller(drive: u8) -> u8 { (drive >> 1) & 1 }

#[inline]
fn channel(drive: u8) -> u8 { drive & 1 }

#[inline]
fn port(drive: u8) -> u16 {
    PORTS[controller(drive) as usize]
}

#[inline]
fn get_status(drive: u8) -> u8 {
    inb(port(drive) + 7)
}

const BSY: u8 = 0x80;
const RDY: u8 = 0x40;
const DRQ: u8 = 0x8;

#[inline]
fn wait_for_drive(drive: u8) {
    while (get_status(drive) & BSY) != 0 {}
    while (get_status(drive) & RDY) == 0 {}
}

fn wait_for_data(drive: u8) {
    wait_for_drive(drive);
    while (get_status(drive) & DRQ) == 0 {}
}

// Returns the number of LBA32 sectors on the drive, if there is one
pub fn sector_count(drive: u8) -> Option<u32> {
    let base = port(drive);
    let channel = channel(drive);

    outb(base + 6, 0xA0 | (channel << 4));
    // Exclusive range
    for i in 2..6 {
        outb(base + i, 0);
    }
    outb(base + 7, 0xEC);

    if get_status(drive) == 0 {
        return None;
    }
    wait_for_data(drive);
    let mut sector_count = 0;
    for i in 0..128 { // 128 32-bit integers on port
        let result = inl(base);
        if i == 30 {
            sector_count = result;
        }
    }
    Some(sector_count)
}

pub fn read_sectors(drive: u8, start_sector: u32, buffer: &mut [u32]) {
    let base = port(drive);
    let channel = channel(drive);

    let longs = buffer.len() as u32;
    let num_sectors = longs >> 7;
    let mut sector = start_sector;
    let end_sector = start_sector + num_sectors;
    let mut buffer_idx = 0;
    while sector < end_sector {
        // Read as many sectors as you can
        let sector_count = (end_sector - sector) as u8;
        outb(base + 2, sector_count);
        outb(base + 3, sector as u8);
        outb(base + 4, (sector >> 8) as u8);
        outb(base + 5, (sector >> 16) as u8);
        outb(base + 6, 0xE0 | (channel << 4) | (((sector >> 24) & 0xf) as u8));
        outb(base + 7, 0x20);

        for _ in 0..sector_count {
            wait_for_data(drive);
            for _ in 0..(SECTOR_BYTES / ::core::mem::size_of::<u32>()) {
                buffer[buffer_idx] = inl(base);
                buffer_idx += 1;
            }
        }
        sector += sector_count as u32;
    }
}

// You have to manually free the pointer when you stop caring
pub fn slurp_drive(drive: u8) -> Option<Vec<u32>> {
    sector_count(drive).map(|count| {
        let bytes = (count as usize) * SECTOR_BYTES;
        let longs = bytes >> 2;
        unsafe {
            let pointer = allocate(bytes, 4096) as *mut u32; // Code should be page-aligned
            read_sectors(drive, 0, slice::from_raw_parts_mut(pointer, longs));
            Vec::from_raw_parts(pointer, longs, longs)
        }
    })
}
