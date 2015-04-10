#![feature(no_std,lang_items,asm,core)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;

use core::prelude::*;

extern crate rlibc;

mod machine;
mod vga;
mod mmu;
mod idt;
mod interrupts;
mod malloc;

use vga::Color::*;

pub use idt::{idtDesc};
pub use interrupts::{double_fault_handler, gpf_handler,
                     kbd_interrupt_handler };
pub use malloc::{rust_allocate, };

#[lang="start"]
#[no_mangle]
pub fn k_main() {
    idt::init();
    interrupts::init();
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
    unsafe {
        asm! {
            "int $0\n"
                :: "N"(0x50)
        }
    }
    loop {};
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
