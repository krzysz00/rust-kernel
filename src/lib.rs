#![feature(no_std,lang_items,asm,core,alloc,collections)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;
extern crate alloc;
extern crate collections;

use core::prelude::*;
use alloc::boxed::Box;
use collections::string::String;

extern crate rlibc;

#[macro_use]
mod notex;

mod machine;
mod vga;
mod mmu;
mod gdt;
mod idt;
mod interrupts;
mod paging;
mod malloc;

use vga::Color::*;

pub use idt::{idtDesc};
pub use interrupts::{double_fault_handler, gpf_handler,
                     page_fault_handler, kbd_interrupt_handler };
pub use malloc::{rust_allocate, rust_reallocate, rust_reallocate_inplace,
                 rust_deallocate, rust_usable_size, rust_stats_print };

#[lang="start"]
#[no_mangle]
pub fn k_main() {
    interrupts::init();
    // YOU MAY NOW PAGE FAULT
    let greet = "Hello from bare-bones Rust";

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
    unsafe {
        *(0x0000_b000_0000 as *mut u64) = 0xDEAD_CAFE_CAFE_F00F;
//        *(0x1234_5678_9ABC as *mut u64) = 0xDEAD_CAFE_CAFE_F00F;
    }
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[inline(always)]
fn panic_fmt() -> ! { unsafe { ::core::intrinsics::abort() } }
