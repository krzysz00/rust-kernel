use machine::{inb};
use paging;
use vga;
use super::apic;
use super::context::{RawContext, RawErrContext, Context, Contextable};
use console::Console;

use core::slice;
use vga::Color::*;

#[no_mangle]
pub extern fn double_fault_handler(_ctx: &mut RawContext) {
    vga::write_string_with_color(0, 20, "Double fault!!!", LightRed, Green);
}

#[no_mangle]
pub extern fn gpf_handler(code: u32, _ctx: &mut RawErrContext) {
    log!("General protection fault: code {}\r\n", code);
}

#[no_mangle]
pub extern fn page_fault_handler(address: u32, error: u32, _ctx: &mut RawErrContext) {
    if (error & 0x1) == 1 { // It's not a missing page?
        log!("Unusual page fault: code 0x{:x}, address: 0x{:x}\r\n", error, address);
        loop {};
    }
    paging::make_present(address as usize);
}

#[no_mangle]
pub extern fn kbd_interrupt_handler(_ctx: &mut RawContext) {
    let _byte = inb(0x60);
    vga::write_string_with_color(4, 30, "Interrupts on!", Pink, Black);
    apic::eoi();
}

#[no_mangle]
pub extern fn write_handler(head: *const u8, len: u32, _ctx: &mut RawContext) {
    let bytes = unsafe { slice::from_raw_parts(head, len as usize) };
    Console.write_bytes(bytes);
}

#[no_mangle]
pub extern fn exit_handler(code: u32, _unused: u32, _ctx: &mut RawContext) {
    log!("Process exited with code {}\r\n", code);
    loop {}
}
