use core::prelude::*;

#[link(name = "asmcode", repr="static")]
extern {
    fn _syscall(number: u32, arg1: u32, arg2: u32) -> u32;
}

fn syscall(number: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe { _syscall(number, arg1, arg2) }
}

pub fn write(bytes: &[u8]) -> bool {
    let ptr = bytes.as_ptr();
    let len = bytes.len();
    let ret = syscall(1, ptr as u32, len as u32);
    ret == 0
}

pub fn sleep() {
    syscall(8, 0, 0);
}

pub fn exit(code: u32) -> ! {
    syscall(9, code, 0);
    loop {};
}
