use mmu::{Descriptor, TableDescriptor};
use core::mem::size_of;

const IDT_COUNT: usize = 256;

#[no_mangle]
pub static mut idtDesc: TableDescriptor =
    TableDescriptor{ limit: 0 , base: 0};

static mut idt: [Descriptor; IDT_COUNT] =
    [ Descriptor { f0: 0, f1: 0 } ; IDT_COUNT];

extern {
    fn lidt2();
}

pub fn init() {
    unsafe {
        idtDesc.limit = (size_of::<Descriptor>() * IDT_COUNT - 1) as u16;
        idtDesc.base = &idt as *const Descriptor as u32;
        lidt2();
    }
}

pub fn register_interrupt(number: usize, handler: unsafe extern fn()) {
    unsafe {
        idt[number].set_trap_descriptor(0x08, handler as u32, 0);
    }
}
