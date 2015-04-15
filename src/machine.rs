#[link(name = "asmcode", repr="static")]
extern {
    fn _inb(port: u32) -> u32;
    fn _inl(port: u32) -> u32;

    fn _outb(port: u32, val: u32);
    fn _ltr(tr: u32);
}

pub fn inb(port: u16) -> u8 {
    unsafe {
        _inb(port as u32) as u8
    }
}

pub fn inl(port: u16) -> u32 {
    unsafe {
        _inl(port as u32)
    }
}

pub fn outb(port: u16, byte: u8) {
    unsafe {
        _outb(port as u32, byte as u32)
    }
}

pub fn ltr(value: u32) {
    unsafe { _ltr(value) }
}
