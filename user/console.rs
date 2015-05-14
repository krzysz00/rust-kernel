use syscall::write;

use core::prelude::*;
use core::fmt::{Write,Error};

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, data: &str) -> Result<(), Error> {
        let result = write(data.as_bytes());
        if result { Result::Ok(()) } else { Result::Err(Error) }
    }
}

pub fn puts(string: &str) {
    let _ = Console.write_str(string);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use ::core::fmt::Write;
        let _ = write!($crate::console::Console, $($arg)*);
    })
}
