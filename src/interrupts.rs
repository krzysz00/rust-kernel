#[no_mangle]

use idt;
use vga;

use core::prelude::*;
use vga::Color::*;

#[link(name="asmcode", repr="static")]
extern {
    fn double_fault_wrapper();
    fn gpf_wrapper();
    fn kbd_interrupt_wrapper();
}

#[no_mangle]
pub extern fn double_fault_handler() {
    vga::write_string_with_color(0, 20, "Double fault!!!", LightRed, Green);
}

#[no_mangle]
pub extern fn gpf_handler(code: u32) {
    const ERROR: &'static str = "General protection fault: ";
    vga::write_string(15, 0, ERROR);
    let repr = ((code >> 3) + 0x21) as u8;
    vga::write_char(15, ERROR.len(), repr as char);
}

#[no_mangle]
pub extern fn kbd_interrupt_handler() {
    vga::write_string_with_color(4, 30, "Interrupts on!", Pink, Black);
}

pub fn init() {
    idt::register_interrupt(0x8, double_fault_wrapper);
    idt::register_interrupt(0xD, gpf_wrapper);
    idt::register_interrupt(0x50, kbd_interrupt_wrapper);
}
