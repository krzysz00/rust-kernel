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

pub static ROWS: usize = 25;
pub static COLS: usize = 80;

fn set_vga_u8(offset: usize, val: u8) {
    unsafe {
        let ptr = (0xb8000 + offset) as *mut u8;
        *ptr = val;
    }
}

fn set_vga_u16(offset: usize, val: u16) {
    unsafe {
        let ptr = (0xb8000 + (offset * 2)) as *mut u16;
        *ptr = val;
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
