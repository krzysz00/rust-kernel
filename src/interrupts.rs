#[no_mangle]

use idt;
use machine::{outb, inb};
use paging;
use vga;

use core::prelude::*;
use vga::Color::*;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

#[link(name="asmcode", repr="static")]
extern {
    fn double_fault_wrapper();
    fn gpf_wrapper();
    fn page_fault_wrapper();
    fn kbd_interrupt_wrapper();
}

#[no_mangle]
pub extern fn double_fault_handler() {
    vga::write_string_with_color(0, 20, "Double fault!!!", LightRed, Green);
}

#[no_mangle]
pub extern fn gpf_handler(code: u64) {
    const ERROR: &'static str = "General protection fault: ";
    vga::write_string(15, 0, ERROR);
    let repr = ((code >> 3) + 0x21) as u8;
    vga::write_char(15, ERROR.len(), repr as char);
}

#[no_mangle]
pub extern fn page_fault_handler(address: u64, error: u64) {
    if (error & 0x1) == 1 { // It's not a missing page?
        vga::write_string(0, 0, "Weird page fault");
        loop {};
    }
    paging::make_present(address);
}

#[no_mangle]
pub extern fn kbd_interrupt_handler() {
    vga::write_string_with_color(4, 30, "Interrupts on!", Pink, Black);
}

fn remap_pic() {
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    outb(PIC1_CMD, 0x11);
    outb(PIC2_CMD, 0x11);

    outb(PIC1_DATA, 0x20);
    outb(PIC2_DATA, 0x28);

    outb(PIC1_DATA, 0x04);
    outb(PIC2_DATA, 0x02);

    outb(PIC1_DATA, 0x01);
    outb(PIC2_DATA, 0x01);

    // Everything is remapped
    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

pub fn mask_pic(master_mask: u8, slave_mask: u8) {
    outb(PIC1_DATA, master_mask);
    outb(PIC2_DATA, slave_mask);
}

// Remaps the PIC, masks everything
pub fn init() {
    idt::init();

    remap_pic();
    mask_pic(0xff, 0xff);

    idt::register_interrupt(0x8, double_fault_wrapper);
    idt::register_interrupt(0xD, gpf_wrapper);
    idt::register_interrupt(0xE, page_fault_wrapper);
    idt::register_interrupt(0x50, kbd_interrupt_wrapper);

    unsafe { asm!("sti" :::: "volatile") }
}
