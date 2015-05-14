use vga;
use machine::{to_user_mode, get_esp};
use paging::give_to_user;

pub extern fn user_main() {
    vga::write_string(0, 0, "User mode");
    loop {}
}

pub fn init() -> ! {
    give_to_user(get_esp() as usize);
    for page in 0x8..0xe0 {
        give_to_user(page << 12);
    }
    to_user_mode(user_main)
}
