use core::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use interrupts::apic::send_interrupt;
use collections::Vec;

extern {
    fn smp_init_vector();
}

#[allow(non_upper_case_globals)]
pub static processor_count: AtomicUsize = ATOMIC_USIZE_INIT;
