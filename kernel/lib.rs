#![feature(no_std,lang_items,asm,core,alloc,collections,step_by)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;
extern crate alloc;
extern crate collections;

use core::prelude::*;
use core::fmt;
use core::atomic::Ordering;
use alloc::boxed::Box;
use alloc::arc::Arc;
use collections::String;

extern crate rlibc;

#[macro_use]
mod mutex;
#[macro_use]
mod lazy_global;

mod machine;
#[macro_use]
mod console;

mod vga;
mod mmu;
mod tasks;
mod acpi;
mod interrupts;
mod paging;
mod malloc;
mod ide;
mod smp;
mod user_mode;

use vga::Color::*;

pub use interrupts::{idt_desc};
pub use interrupts::handlers::{double_fault_handler, gpf_handler,
                               page_fault_handler, broadcast_timer_handler,
                               timer_handler, kbd_interrupt_handler,
                               write_handler, sleep_handler, exit_handler };
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
        interrupts::init();
        smp::init(Arc::new(smp::Globals {
            processors: acpi::processor_list(),
            bsp: interrupts::apic::id(),
            the_code: ide::slurp_drive(1),
        }));
        tasks::init();

        let greet = "Hello from bare-bones Rust";
        console::puts("Hello");
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
        let processors = smp::processor_count.load(Ordering::SeqCst) as u8 + '0' as u8;
        vga::write_char(4, 4, processors as char);
        user_mode::init();
    }
    else { // Non-main processor
        interrupts::apic::enable_lapic();
        let id = interrupts::apic::id();
        let globals = smp::globals();
        let bsp_id = globals.bsp;
        log!("I am {}. The main processor is {}\r\n", id, bsp_id);
        if tasks::init() {
            user_mode::init();
        }
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind(args: fmt::Arguments,
                                file: &'static str, line: u32) -> ! {
    use core::fmt::Write;
    log!("\r\nPanic at {}:{}: ", file, line);
    let _ = console::Console.write_fmt(args);
    loop {};
}
