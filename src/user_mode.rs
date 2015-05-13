use vga;
use machine::{to_user_mode, get_esp, syscall};
use paging::give_to_user;

use core::fmt::{Write,Error};
use core::result::Result;

pub struct UserConsole;

impl Write for UserConsole {
    fn write_str(&mut self, data: &str) -> Result<(), Error> {
        let ptr = data.as_ptr();
        let len = data.len();
        let ret = syscall(1, ptr as u32, len as u32);
        if ret == 0 { Result::Ok(()) } else { Result::Err(Error) }
    }
}

pub extern fn user_main() {
    vga::write_string(0, 0, "User mode");
    match UserConsole.write_str("\r\nSyscall test\r\n") {
        Result::Ok(_) => vga::write_string(0, 30, "Post-interrupt"),
        _ => ()
    }
    loop {}
}

pub fn init() -> ! {
    give_to_user(get_esp() as usize);
    for page in 0x8..0xe0 {
        give_to_user(page << 12);
    }
    to_user_mode(user_main)
}
