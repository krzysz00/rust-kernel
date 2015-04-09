#[link(name = "asmcode", repr="static")]
extern {
    fn inb(port: u32) -> u32;
    fn inl(port: u32) -> u32;

    fn outb(port: u32, val: u32);
    fn ltr(tr: u32);
}

pub fn read_byte(port: u16) -> u8 {
    unsafe {
        inb(port as u32) as u8
    }
}

pub fn read_long(port: u16) -> u32 {
    unsafe {
        inb(port as u32) as u32
    }
}

pub fn write_port(port: u16, byte: u8) {
    unsafe {
        outb(port as u32, byte as u32)
    }
}

pub fn set_task_register(value: u32) {
    unsafe { ltr(value) }
}
