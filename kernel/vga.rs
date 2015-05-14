#[derive(Copy, Clone)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

use machine;

use core::prelude::*;

pub const ROWS: usize = 25;
pub const COLS: usize = 80;

// TODO: Re-enable this once something pulls in bounds checking
// static mut vga_mem: *mut [u8; ROWS * COLS * 2] = 0xb8000 as *mut [u8; ROWS * COLS * 2];
// static mut vga_mem_long: *mut [u16; ROWS * COLS] = 0xb8000 as *mut [u16; ROWS * COLS];

fn set_vga_u8(offset: usize, val: u8) {
    unsafe {
        *(0xb8000 as *mut u8).offset(offset as isize) = val;
    }
}

fn set_vga_u16(offset: usize, val: u16) {
    unsafe {
        *(0xb8000 as *mut u16).offset(offset as isize) = val;
    }
}

pub fn write_char(row: usize, col: usize, letter: char) {
    let code = (letter as u32) as u8;
    let offset = (row * COLS + col) * 2;
    set_vga_u8(offset, code);
}

fn encode_color(foregnd: Color, backgnd: Color) -> u8 {
    let fcode = foregnd as u8;
    let bcode = backgnd as u8;
    fcode | (bcode << 4)
}

pub fn set_color(row: usize, col: usize, foregnd: Color, backgnd: Color) {
    let code = encode_color(foregnd, backgnd);
    let offset = (row * COLS + col) * 2 + 1;
    set_vga_u8(offset, code);
}

pub fn write_char_with_color(row: usize, col: usize, letter: char,
                             foregnd: Color, backgnd: Color) {
    let char_code = (letter as u32) as u16;
    let color_code = encode_color(foregnd, backgnd) as u16;
    let offset = (row * COLS) + col;
    let code = (char_code & 0xFF) | (color_code << 8);
    set_vga_u16(offset, code);
}

pub fn write_string(row: usize, col: usize, string: &str) {
    for (i, chr) in string.bytes().enumerate() {
        write_char(row, col + i, chr as char);
    }
}

pub fn paint_color(row: usize, col: usize, len: usize,
                   foregnd: Color, backgnd: Color) {
    for i in 0..len {
        set_color(row, col + i, foregnd, backgnd);
    }
}

pub fn write_string_with_color(row: usize, col: usize, string: &str,
                               foregnd: Color, backgnd: Color) {
    for (i, chr) in string.bytes().enumerate() {
        write_char_with_color(row, col + i, chr as char, foregnd, backgnd);
    }
}

pub fn move_cursor(row: usize, col: usize) {
    const VGA_CMD: u16 = 0x3d4;
    const VGA_DATA: u16 = 0x3d5;
    let cursor_offset = (row * COLS + col) as u16;
    let lsb = (cursor_offset & 0xFF) as u8;
    let msb = (cursor_offset >> 8) as u8;

    machine::outb(VGA_CMD, 0x0f);
    machine::outb(VGA_DATA, lsb);
    machine::outb(VGA_CMD, 0x0e);
    machine::outb(VGA_DATA, msb);
}
