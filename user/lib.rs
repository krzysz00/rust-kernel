#![feature(lang_items)]
#![feature(alloc,collections)]
#![no_std]

#![crate_type="staticlib"]
#![crate_name="rustcode"]

#[macro_use]
extern crate alloc_shim;
extern crate alloc;
extern crate collections;

use core::fmt;

extern crate rlibc;

mod syscall;
#[macro_use]
mod console;

const MAX_PRIME: u32 = 10000;

#[lang="start"]
#[no_mangle]
pub fn main(pid: u32) {
    print!("I am process {}\r\n", pid);
    console::puts("Searching for primes\r\n");
    for i in 2..MAX_PRIME {
        let mut is_prime = true;
        for j in 2..i {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            print!("{} is prime\r\n", i);
        }
        // if i % 100 == 0 {
        //     syscall::sleep();
        // }
    }
}

//#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(args: fmt::Arguments,
                                file: &'static str, line: u32) -> ! {
    use core::fmt::Write;
    print!("\r\nPanic at {}:{}: ", file, line);
    let _ = console::Console.write_fmt(args);
    syscall::exit(1);
}
