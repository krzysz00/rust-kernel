#![feature(no_std,lang_items,asm,core)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;

use core::prelude::*;
use core::mem;

extern crate rlibc;

mod machine;
mod vga;
mod mmu;
mod idt;

use vga::Color::*;

pub use idt::{idtDesc};

#[lang="start"]
#[no_mangle]
pub fn k_main() {
    idt::init();
    for i in 0..vga::ROWS {
        for j in 0..vga::COLS {
            vga::write_char_with_color(i, j, ' ', White, LightBlue);
        }
    }
    vga::set_color(0, 0, White, LightGreen);
    vga::write_char(10, 10, 'd');
    vga::move_cursor(5, 77);
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
