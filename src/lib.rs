#![feature(no_std,lang_items,asm,core)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate core;

// use core::prelude::*;

extern crate rlibc;

#[lang="start"]
#[no_mangle]
pub fn k_main() {
    loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
