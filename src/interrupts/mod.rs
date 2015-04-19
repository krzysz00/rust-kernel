mod idt;
mod pic;
pub mod apic;

pub use self::idt::{idtDesc};

use acpi::IOAPIC;
use collections::Vec;

use machine::{inb};
use paging;
use vga;

use core::prelude::*;
use vga::Color::*;

#[link(name="asmcode", repr="static")]
extern {
    fn double_fault_wrapper();
    fn gpf_wrapper();
    fn page_fault_wrapper();
    fn kbd_interrupt_wrapper();
    fn spurious_interrupt_handler();
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
pub extern fn page_fault_handler(address: u32, error: u32) {
    if (error & 0x1) == 1 { // It's not a missing page?
        vga::write_string(0, 0, "Weird page fault");
        loop {};
    }
    paging::make_present(address);
}

#[no_mangle]
pub extern fn kbd_interrupt_handler() {
    let byte = inb(0x60);
    vga::write_string_with_color(4, 30, "Interrupts on!", Pink, Black);
    apic::eoi();
}

// Remaps the PIC, masks everything
pub fn init_idt() {
    idt::init();

    pic::remap_pic();
    pic::mask_pic(0xff, 0xff);

    idt::register_interrupt(0x8, double_fault_wrapper);
    idt::register_trap(0xD, gpf_wrapper);
    idt::register_trap(0xE, page_fault_wrapper);
    idt::register_interrupt(0x21, kbd_interrupt_wrapper);
    idt::register_interrupt(0xFF, spurious_interrupt_handler);
}

pub fn init(info: &Vec<IOAPIC>) {
    apic::init(info);
    unsafe { asm!("sti" :::: "volatile") }
}
