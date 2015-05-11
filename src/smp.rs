use core::prelude::*;
use core::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use alloc::arc::Arc;
use collections::Vec;

use interrupts::apic;
use machine::outb;
use notex::Notex;
use acpi;

extern {
    fn smp_init_vector();
}

#[allow(non_upper_case_globals)]
pub static processor_count: AtomicUsize = ATOMIC_USIZE_INIT;

#[no_mangle]
pub static SMP_STACK_PTR: AtomicUsize = ATOMIC_USIZE_INIT;
#[no_mangle]
pub static SMP_CR3: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct SMPInfo {
    pub processors: Vec<u8>,
    pub bsp: u8,
}

// I'm satisfied that there are no race conditions (Arc is atomic)
// and I don't want the overhead
static SMP_INFO: Notex<Option<Arc<SMPInfo>>> = notex!(None);

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

pub fn get_smp_info() -> Arc<SMPInfo> {
    let maybe_info = SMP_INFO.lock();
    match *maybe_info {
        Some(ref arc) => arc.clone(),
        None => panic!("SMP should have been initialized"),
    }
}

pub fn init() {
    let (self_id, is_bsp) = apic::whoami();
    if is_bsp {
        let processors = acpi::processor_list();
        {
            let mut lock = SMP_INFO.lock();
            *lock = Some(Arc::new(SMPInfo {
                processors: processors,
                bsp: self_id,}));
        }
        // We just gave up the vector, so we get a new pointer back
        let processors = &get_smp_info().processors;

        apic::set_ioapic_id(processors.len() as u8);
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
