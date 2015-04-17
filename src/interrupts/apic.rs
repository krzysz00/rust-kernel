use machine::{rdmsr, wrmsr};
use paging::{identity_map};

const LAPIC_MSR_ID: u32 = 0x1B;
const LAPIC_ENABLE_BIT: u64 = 1 << 11;
const LAPIC_BSP_BIT: u64 = 1 << 8;

const LAPIC_BASE: usize = 0xFEE0_0000;
const IOAPIC_BASE: usize = 0xFEC0_0000;

fn read_lapic_reg(reg: u32) -> u32 {
    let addr = (LAPIC_BASE as u32) | reg;
    unsafe { *(addr as *const u32) }
}

fn write_lapic_reg(reg: u32, val: u32) {
    let addr = (LAPIC_BASE as u32) | reg;
    unsafe { *(addr as *mut u32) = val; }
}

pub fn enable_lapic() {
    let val = rdmsr(LAPIC_MSR_ID);
    let val = (val & 0xffff_f100) | LAPIC_ENABLE_BIT;
    wrmsr(LAPIC_MSR_ID, val);

    write_lapic_reg(0xF0, 0x1ff);
}

pub fn whoami() -> (u32, bool) {
    let msr = rdmsr(LAPIC_MSR_ID);
    let id = read_lapic_reg(0x20);
    (id >> 24, (msr & LAPIC_BSP_BIT) != 0)
}

pub fn eoi() {
    write_lapic_reg(0xB0, 0);
}

fn read_ioapic_reg(reg: u8) -> u32 {
    let ioapic = IOAPIC_BASE as *mut u32;
    let reg = reg as u32;
    unsafe {
        *ioapic = reg;
        *ioapic.offset(4)
    }
}

fn write_ioapic_reg(reg: u8, value: u32) {
    let ioapic = IOAPIC_BASE as *mut u32;
    let reg = reg as u32;
    unsafe {
        *ioapic = reg;
        *ioapic.offset(4) = value;
    }
}

pub fn direct_irq(irq: u8, vector: u8, dest: u32) {
    let f0 = vector as u32; // All other bits should be 0
    let f1 = dest << 24;
    let irq_reg = 0x10 + (2 * irq);
    write_ioapic_reg(irq_reg, f0);
    ::core::atomic::fence(::core::atomic::Ordering::Acquire);
//    if read_ioapic_reg(irq_reg) != 0 {
    write_ioapic_reg(irq_reg + 1, f1);
//    }
}

pub fn mask_irq(irq: u8) {
    let irq_reg = 0x10 + (2 * irq);
    let direction = read_ioapic_reg(irq_reg);
    write_ioapic_reg(irq_reg, direction | (1 << 16));
}

pub fn send_interrupt(lapic_id: u32, irq: u32) {
    write_lapic_reg(0x310, lapic_id << 24);
    write_lapic_reg(0x300, irq);
}

pub fn init() {
    identity_map(LAPIC_BASE);
    identity_map(IOAPIC_BASE);

    enable_lapic();
    let (lapic_id, is_bsp) = whoami();
    if is_bsp {
        direct_irq(0x1, 0x21, lapic_id);
    }
}
