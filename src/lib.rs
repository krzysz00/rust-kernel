#![feature(no_std,lang_items,asm,core)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;

use core::prelude::*;

extern crate rlibc;

mod vga;

use vga::Color::*;

#[lang="start"]
#[no_mangle]
pub fn k_main() {
    for i in 0..vga::ROWS {
        for j in 0..vga::COLS {
            vga::write_char_with_color(i, j, ' ', White, LightBlue);
        }
    }
    vga::set_color(0, 0, White, LightGreen);
    vga::write_char(10, 10, 'd');
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
