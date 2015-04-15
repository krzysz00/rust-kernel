use mmu::{Descriptor, TableDescriptor};
use core::mem::size_of;
use notex::Notex;

const IDT_COUNT: usize = 256;

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static idtDesc: Notex<TableDescriptor> =
    notex!(TableDescriptor{ limit: 0 , base: 0});

static IDT_NOTEX: Notex<[Descriptor; IDT_COUNT]> =
    notex!([ Descriptor { f0: 0, f1: 0, f2: 0, f3: 0 } ; IDT_COUNT]);

extern {
    fn lidt2();
}

pub fn init() {
    unsafe {
        let idt = IDT_NOTEX.lock();
        let idt_addr = &(idt[0]) as *const Descriptor as usize;
        let mut idt_desc = idtDesc.lock();
        idt_desc.limit = (size_of::<Descriptor>() * IDT_COUNT - 1) as u16;
        idt_desc.base = idt_addr as u64;
        lidt2();
    }
}

pub fn register_interrupt(number: usize, handler: unsafe extern fn()) {
    let mut idt = IDT_NOTEX.lock();
    idt[number].set_interrupt_descriptor(0x08, handler as u64, 0);
}
