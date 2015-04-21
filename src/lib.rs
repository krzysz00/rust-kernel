#![feature(no_std,lang_items,asm,core,alloc,collections,step_by)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;
extern crate alloc;
extern crate collections;

use core::prelude::*;
use core::fmt::Write;
use core::atomic::Ordering;
use alloc::boxed::Box;
use collections::string::String;

extern crate rlibc;

#[macro_use]
mod notex;
#[macro_use]
mod mutex;

mod machine;
mod console;
mod vga;
mod mmu;
mod gdt;
mod acpi;
mod interrupts;
mod paging;
mod malloc;
mod smp;

use vga::Color::*;

pub use interrupts::{idtDesc};
pub use interrupts::{double_fault_handler, gpf_handler,
                     page_fault_handler, kbd_interrupt_handler };
pub use malloc::{rust_allocate, rust_reallocate, rust_reallocate_inplace,
                 rust_deallocate, rust_usable_size, rust_stats_print };
pub use smp::{SMP_STACK_PTR, SMP_CR3};

#[lang="start"]
#[no_mangle]
pub fn k_main() {
    if smp::processor_count.fetch_add(1, Ordering::SeqCst) == 0 {
        paging::init();
        interrupts::init_idt();
        // YOU MAY NOW PAGE FAULT
        let smp_info = acpi::smp_info();
        interrupts::init(&(*smp_info).io_apics);
        smp::init(&(*smp_info).processors);

        let greet = "Hello from bare-bones Rust";
        let _ = write!(console::Console, "SMP at {:p}", smp_info);
//        let _ = console::Console.write_str("Hello");
        for i in 0..vga::ROWS {
            for j in 0..vga::COLS {
            vga::write_char_with_color(i, j, ' ', White, LightBlue);
            }
        }
        vga::paint_color(0, 0, 30, White, LightGreen);
        vga::write_string(10, 5, greet);
        vga::move_cursor(10, 5 + greet.len());
        vga::write_string_with_color(11, 10, "Test", Black, LightRed);

        let heap = Box::new('H');
        let heap2 = Box::new('!');
        vga::write_char(15,4,*heap);
        vga::write_char_with_color(15, 5, *heap2, LightGray, Pink);

        let mut string = String::from_str("Hello, ");
        string.push_str("paging");
        vga::write_string(3,5,&string);
        unsafe { *(0xB0_00_00_01 as *mut u32) = 0xcafecafe; }
        unsafe { *(0xA0_00_10_00 as *mut mmu::Descriptor) = gdt::gdt_get(1); }
        let processors = smp::processor_count.load(Ordering::SeqCst) as u8 + '0' as u8;
        vga::write_char(4, 4, processors as char);
    }
    else { // Non-main processor
        interrupts::apic::enable_lapic();
        loop {};
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[inline(always)]
fn panic_fmt() -> ! { unsafe { ::core::intrinsics::abort() } }
