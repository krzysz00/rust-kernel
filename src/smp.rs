use core::prelude::*;
use core::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use interrupts::apic;
use collections::Vec;
use machine::{outb};

extern {
    fn smp_init_vector();
}

#[allow(non_upper_case_globals)]
pub static processor_count: AtomicUsize = ATOMIC_USIZE_INIT;

#[no_mangle]
pub static SMP_STACK_PTR: AtomicUsize = ATOMIC_USIZE_INIT;
#[no_mangle]
pub static SMP_CR3: AtomicUsize = ATOMIC_USIZE_INIT;

fn send_startup_interrupt(address: u32, id: u8) {
    outb(0x70, 0x0F);
    outb(0x71, 0x0A);
    unsafe { ::core::intrinsics::volatile_store(0x469 as *mut u32, address); }
    let interrupt = 0x4500;
    apic::send_interrupt(id, interrupt);
    apic::wait_for_delivery();
    let interrupt = (address >> 12) | 0x4600;
    apic::send_interrupt(id, interrupt);
    apic::wait_for_delivery();
}

pub fn init(processors: &Vec<u8>) {
    let (self_id, is_bsp) = apic::whoami();
    if is_bsp {
        let startup_address: u32 = smp_init_vector as u32;
        for (number, id) in processors.iter().enumerate() {
            let stack_loc = 0x120_000 + (8192 * number);
            SMP_STACK_PTR.store(stack_loc, Ordering::SeqCst);
            if *id != self_id {
                unsafe {
                    // Trigger the page fault in advance
                    *(stack_loc as *mut u32) = 0xDEADBEEF;
                    *((stack_loc - 4) as *mut u32) = 0;
                    *((stack_loc - 0x1000 - 4) as *mut u32) = 0;
                }
                let old_processors_count = processor_count.load(Ordering::Relaxed);
                send_startup_interrupt(startup_address, *id);
                while processor_count.load(Ordering::SeqCst) <= old_processors_count { };
            }
        }
    }
}
