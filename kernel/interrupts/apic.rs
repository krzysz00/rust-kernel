use machine::{rdmsr, wrmsr};
use paging::{identity_map};

use core::intrinsics::{volatile_load, volatile_store};

const LAPIC_MSR_ID: u32 = 0x1B;
const LAPIC_ENABLE_BIT: u64 = 1 << 11;
const LAPIC_BSP_BIT: u64 = 1 << 8;

const LAPIC_BASE: usize = 0xFEE0_0000;
const IOAPIC_BASE: usize = 0xFEC0_0000;

fn read_lapic_reg(reg: u32) -> u32 {
    let addr = (LAPIC_BASE as u32) | reg;
    unsafe { volatile_load(addr as *const u32) }
}

fn write_lapic_reg(reg: u32, val: u32) {
    let addr = (LAPIC_BASE as u32) | reg;
    unsafe { volatile_store(addr as *mut u32,  val); }
}

pub fn enable_lapic() {
    let val = rdmsr(LAPIC_MSR_ID);
    let val = (val & 0xffff_f100) | LAPIC_ENABLE_BIT;
    wrmsr(LAPIC_MSR_ID, val);

    write_lapic_reg(0xF0, 0x1ff);
}

#[inline]
pub fn id() -> u8 {
    (read_lapic_reg(0x20) >> 24) as u8
}

#[inline]
pub fn is_bsp() -> bool {
    (rdmsr(LAPIC_MSR_ID) & LAPIC_BSP_BIT) != 0
}

pub fn whoami() -> (u8, bool) {
    (id(), is_bsp())
}

pub fn eoi() {
    write_lapic_reg(0xB0, 0);
}

fn read_ioapic_reg(reg: u8) -> u32 {
    let ioapic = IOAPIC_BASE as *mut u32;
    let reg = reg as u32;
    unsafe {
        volatile_store(ioapic, reg);
        volatile_load(ioapic.offset(4))
    }
}

fn write_ioapic_reg(reg: u8, value: u32) {
    let ioapic = IOAPIC_BASE as *mut u32;
    let reg = reg as u32;
    unsafe {
        volatile_store(ioapic, reg);
        volatile_store(ioapic.offset(4), value);
    }
}

pub fn set_ioapic_id(id: u8) {
    write_ioapic_reg(0x0, (id as u32) << 24 | read_ioapic_reg(0x0));
}

pub fn direct_irq(irq: u8, vector: u8, dest: u32) {
    let f0 = vector as u32; // All other bits should be 0
    let f1 = dest << 24;
    let irq_reg = 0x10 + (2 * irq);
    write_ioapic_reg(irq_reg, f0);
    write_ioapic_reg(irq_reg + 1, f1);
}

pub fn mask_irq(irq: u8) {
    let irq_reg = 0x10 + (2 * irq);
    let direction = read_ioapic_reg(irq_reg);
    write_ioapic_reg(irq_reg, direction | (1 << 16));
}

pub fn send_interrupt(lapic_id: u8, irq: u32) {
    write_lapic_reg(0x310, (lapic_id as u32) << 24);
    write_lapic_reg(0x300, irq);
}

pub fn wait_for_delivery() {
    while read_lapic_reg(0x300) & (1 << 12) != 0 {}
}

pub fn init() {
    identity_map(LAPIC_BASE);
    identity_map(IOAPIC_BASE);

    enable_lapic();
    let (lapic_id, is_bsp) = whoami();
    if is_bsp {
        direct_irq(0x1, 0x21, lapic_id as u32);
        direct_irq(0x2, 0x22, lapic_id as u32);
    }
}
