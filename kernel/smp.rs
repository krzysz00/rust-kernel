use core::prelude::*;
use core::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use alloc::arc::Arc;
use collections::Vec;

use interrupts::apic;
use machine::outb;
use tasks::Tss;
use user_mode::Process;
use lazy_global::LazyGlobal;

extern {
    fn smp_init_vector();
}

#[allow(non_upper_case_globals)]
pub static processor_count: AtomicUsize = ATOMIC_USIZE_INIT;

#[no_mangle]
pub static SMP_STACK_PTR: AtomicUsize = ATOMIC_USIZE_INIT;
#[no_mangle]
pub static SMP_CR3: AtomicUsize = ATOMIC_USIZE_INIT;

pub struct Globals {
    pub processors: Vec<u8>,
    pub bsp: u8,
    pub the_code: Option<Vec<u32>>,
}

// I'm satisfied that there are no race conditions (Arc is atomic)
// and I don't want the overhead
static GLOBALS: LazyGlobal<Arc<Globals>> = lazy_global!();

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

pub fn globals() -> Arc<Globals> {
    // Arc is thread-safe, so we're fine
    unsafe { GLOBALS.get().clone() }
}

#[derive(Default)]
pub struct Locals {
    pub tss: Tss,
    pub process: Option<Process>,
}

// I solemnly swear that each processor only gets its own locals
static PROCESSOR_LOCALS: LazyGlobal<Vec<Locals>> = lazy_global!();

fn init_locals_vector(num_processors: usize) {
    unsafe {
        // Only thread, other people need these
        PROCESSOR_LOCALS.init(Vec::with_capacity(num_processors));
    }
}

fn init_locals(id: u8) {
    unsafe {
        // Only processor
        PROCESSOR_LOCALS.get_mut().insert(id as usize, Default::default());
    }
}

pub fn locals() -> &'static Locals {
    unsafe {
        // You only get yours
        &PROCESSOR_LOCALS.get()[apic::id() as usize]
    }
}

pub fn locals_mut() -> &'static mut Locals {
    unsafe {
        &mut PROCESSOR_LOCALS.get_mut()[apic::id() as usize]
    }
}

pub fn init(info: Arc<Globals>) {
    if apic::is_bsp() {
        unsafe {
            // It's fine, we're the only thread
            GLOBALS.init(info)
        };

        // We just gave up the vector, so we get a new pointer back
        let globals = globals();
        let processors = &globals.processors;
        let self_id = globals.bsp;

        let num_processors = processors.len();
        apic::set_ioapic_id(num_processors as u8);
        init_locals_vector(num_processors);

        let startup_address: u32 = smp_init_vector as u32;
        for (number, id) in processors.iter().enumerate() {
            let stack_loc = 0x120_000 + (8192 * number);
            SMP_STACK_PTR.store(stack_loc, Ordering::SeqCst);
            init_locals(*id);
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
