use core::prelude::*;
use core::mem::size_of;

use mmu::Descriptor;
use smp::locals_mut;
use interrupts::apic;
use machine::{get_esp, ltr};
use mutex::Mutex;

// GDT Layout
// 0 = NULL
// 8 = kernel code
// 0x10 = kernel data
// 0x18 = user code
// 0x20 = user data
// 0x28 ... = TSS entries

const GDT_COUNT: usize = 10;
const GDT_FIXED_ENTRIES: usize = 5;

extern {
    static mut gdt: [Descriptor; GDT_COUNT];
}

// Let's not concurrently modify the GDT
static GDT_MUTEX: Mutex<()> = mutex!(());

pub struct Tss {
    _prev: u32,
    esp0: u32,
    ss0: u32,
    _unused: [u32; 23],
}

impl Default for Tss {
    fn default() -> Tss {
        Tss { _prev: 0, esp0: 0, ss0: 0x10, _unused: [0; 23] }
    }
}

impl Tss {
    #[inline]
    pub fn set_esp0(&mut self, esp0: u32) {
        self.esp0 = esp0
    }
}

// Returns true if the process can go to userspace
pub fn init() -> bool {
    let index = apic::id() as usize + GDT_FIXED_ENTRIES;
    if index >= GDT_COUNT { return false };

    let ref mut tss = *locals_mut().tss;
    // At most 1024 integers on the esp0 stack
    let esp0 = get_esp() - 0x1000;
    tss.set_esp0(esp0);

    let _lock = GDT_MUTEX.lock();
    unsafe {
        gdt[index].set_tss_descriptor(tss as *const Tss as u32,
                                      size_of::<Tss>() as u32, 3);
    }
    ltr((index << 3) as u32 | 0x03); // User mode
    true
}
