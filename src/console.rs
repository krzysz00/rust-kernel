use machine;
use core::fmt::{Write,Error};
use core::result::Result;

const PORT: u16 = 0x3F8;
pub struct Console;

impl Write for Console {
    fn write_str(&mut self, data: &str) -> Result<(), Error> {
        for b in data.bytes() {
            while machine::inb(PORT + 5) & 0x20 == 0 {};
            machine::outb(PORT, b as u8);
        }
        Result::Ok(())
    }
}