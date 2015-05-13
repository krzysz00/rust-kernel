mod idt;
mod pic;
pub mod apic;

pub use self::idt::{idtDesc};

use machine::{inb};
use paging;
use vga;
use console::Console;

use core::slice;
use vga::Color::*;

#[link(name="asmcode", repr="static")]
extern {
    fn double_fault_wrapper();
    fn gpf_wrapper();
    fn page_fault_wrapper();
    fn kbd_interrupt_wrapper();
    fn spurious_interrupt_handler();
    fn syscall_handler();
}

#[no_mangle]
pub extern fn double_fault_handler() {
    vga::write_string_with_color(0, 20, "Double fault!!!", LightRed, Green);
}

#[no_mangle]
pub extern fn gpf_handler(code: u32) {
    log!("General protection fault: code {}\r\n", code);
}

#[no_mangle]
pub extern fn page_fault_handler(address: u32, error: u32) {
    if (error & 0x1) == 1 { // It's not a missing page?
        log!("Unusual page fault: code 0x{:x}, address: 0x{:x}\r\n", error, address);
        loop {};
    }
    paging::make_present(address as usize);
}

#[no_mangle]
pub extern fn kbd_interrupt_handler() {
    let byte = inb(0x60);
    vga::write_string_with_color(4, 30, "Interrupts on!", Pink, Black);
    apic::eoi();
}

#[no_mangle]
pub extern fn write_handler(head: *const u8, len: u32) -> u32 {
    let bytes = unsafe { slice::from_raw_parts(head, len as usize) };
    Console.write_bytes(bytes);
    0
}

pub fn init_idt() {
    idt::init();

    pic::remap_pic();
    pic::mask_pic(0xff, 0xff);

    idt::register_interrupt(0x8, double_fault_wrapper, 0);
    idt::register_trap(0xD, gpf_wrapper, 0);
    idt::register_trap(0xE, page_fault_wrapper, 0);
    idt::register_interrupt(0x21, kbd_interrupt_wrapper, 0);
    idt::register_interrupt(0x50, syscall_handler, 3);
    idt::register_interrupt(0xFF, spurious_interrupt_handler, 0);
}

// Needs the IO APIC to have an ID
pub fn init() {
    apic::init();
    unsafe { asm!("sti" :::: "volatile") }
}
