use mmu::{Descriptor, TableDescriptor};
use core::mem::size_of;
use mutex::Mutex;

const IDT_COUNT: usize = 256;

#[no_mangle]
#[allow(non_upper_case_globals)]
pub static mut idtDesc: TableDescriptor = TableDescriptor{limit: 0 , base: 0};

static IDT_MUTEX: Mutex<[Descriptor; IDT_COUNT]> =
    mutex!([ Descriptor { f0: 0, f1: 0 } ; IDT_COUNT]);

extern {
    fn lidt2();
}

pub fn init() {
    unsafe {
        let idt = IDT_MUTEX.lock();
        let idt_addr = &(idt[0]) as *const Descriptor as usize;
        idtDesc.limit = (size_of::<Descriptor>() * IDT_COUNT - 1) as u16;
        idtDesc.base = idt_addr as u32;
        lidt2();
    }
}

pub fn register_interrupt(number: usize, handler: unsafe extern fn(), dpl: u32) {
    let mut idt = IDT_MUTEX.lock();
    idt[number].set_descriptor(0x08, handler as u32, dpl, 0xE);
}

pub fn register_trap(number: usize, handler: unsafe extern fn(), dpl: u32) {
    let mut idt = IDT_MUTEX.lock();
    idt[number].set_descriptor(0x08, handler as u32, dpl, 0xF);
}
