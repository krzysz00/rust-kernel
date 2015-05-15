use machine::{inb};
use paging;
use vga;
use super::apic;
use super::context::{RawContext, RawErrContext};
use console::Console;
use user_mode;

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
    let address = address as usize;
    if (error & 0x4) != 0 { // User code needs mapping
        let process = user_mode::get_current_process_mut();
        let maybe_code_offset = address - user_mode::USER_LOAD_ADDR;
        let page = maybe_code_offset >> 12;
        let code_pages = (process.code_len >> 10) + if process.code_len % 1024 == 0 { 0 } else { 1 };
        if page > code_pages {
            let frame = process.add_page().1;
            process.pages[0][page] = frame | 0x07; // BSS/stack
        }
        else {
            let frame = paging::frame_for(process.code_addr + maybe_code_offset) as u32;
            process.pages[0][page] = frame | 0x07;
        }
    }
    else {
        paging::make_present(address);
    }
}

#[no_mangle]
pub extern fn broadcast_timer_handler(ctx: &mut RawContext) {
    if !ctx.was_kernel() {
        user_mode::switch_tasks(ctx);
    }
    apic::eoi();
}

#[no_mangle]
pub extern fn timer_handler(ctx: &mut RawContext) {
    // Broadcast the real timer interrupt
    apic::send_interrupt(0xff, 0x49 | 3 << 18);
    broadcast_timer_handler(ctx);
}

#[no_mangle]
pub extern fn kbd_interrupt_handler(_ctx: &mut RawContext) {
    let _byte = inb(0x60);
    vga::write_string_with_color(4, 30, "Interrupts on!", Pink, Black);
    apic::eoi();
}

#[no_mangle]
pub extern fn write_handler(head: *const u8, len: u32, ctx: &mut RawContext) {
    let bytes = unsafe { slice::from_raw_parts(head, len as usize) };
    Console.write_bytes(bytes);
    ctx.set_return_code(0);
}

#[no_mangle]
pub extern fn sleep_handler(_a: u32, _b: u32, ctx: &mut RawContext) {
    ctx.kernel_paging();
    user_mode::switch_tasks(ctx);
}

#[no_mangle]
pub extern fn exit_handler(code: u32, _unused: u32, ctx: &mut RawContext) {
    ctx.kernel_paging();
    log!("Process exited with code {}\r\n", code);
    user_mode::kill_current_process(ctx);
}
