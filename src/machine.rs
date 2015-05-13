#[link(name = "asmcode", repr="static")]
extern {
    fn _ltr(tr: u32);

    fn _rdmsr(id: u32) -> u64;
    fn _wrmsr(id: u32, value: u64);
    fn _enable_paging(page_directory: *const u32);

    fn _invlpg(addr: u32);
}

pub fn inb(port: u16) -> u8 {
    unsafe {
        let value: u8;
        asm!("inb %dx, %al" : "={al}"(value) : "{dx}"(port));
        value
    }
}

pub fn inl(port: u16) -> u32 {
    unsafe {
        let value: u32;
        asm!("inb %dx, %eax" : "={eax}"(value) : "{dx}"(port));
        value
    }
}

pub fn outb(port: u16, byte: u8) {
    unsafe {
        asm!("outb %al, %dx" : : "{dx}"(port), "{al}"(byte))
    }
}

pub fn ltr(value: u32) {
    unsafe { _ltr(value) }
}

pub fn rdmsr(id: u32) -> u64 {
    unsafe { _rdmsr(id) }
}

pub fn wrmsr(id: u32, value: u64) {
    unsafe { _wrmsr(id, value); }
}

pub fn enable_paging(page_directory: *const u32) {
    unsafe {
        _enable_paging(page_directory);
    }
}

pub fn invlpg(vaddr: u32) {
    unsafe {
        _invlpg(vaddr);
    }
}

pub fn get_esp() -> u32 {
    unsafe {
        let ret: u32;
        asm!("mov %esp, $0" : "=r"(ret));
        ret
    }
}
