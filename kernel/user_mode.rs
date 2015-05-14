use machine::to_user_mode;
use paging::{PageTable, frame_for, PAGE_TABLE_SIZE};
use smp::{locals_mut, globals};
use interrupts::Context;
use tasks::Tss;
use console;

use core::prelude::*;
use core::mem::size_of;
use alloc::heap::allocate;
use alloc::boxed::Box;

pub const USER_LOAD_ADDR: usize = 0x400_000;

pub struct Process {
    pub id: u32,
    pub page_tables: Box<[PageTable; 2]>,
    pub code_addr: usize,
    pub code_len: usize,
    pub context: Context,
}

fn create_process(code: &[u32]) -> (usize, usize) {
    let locals = locals_mut();
    let ref mut processes = locals.processes;
    let id = processes.len() + 1;

    let code_addr = &code[0] as *const u32 as usize;
    let code_len = code.len();
    let mut pages = unsafe {
        Box::from_raw(allocate(size_of::<[PageTable; 2]>(), 4096) as *mut [PageTable; 2])
    };
    let system_pd = 0xFF_FF_F000 as *const u32;
    pages[0][0] = unsafe { *(system_pd) };
    let tss_pd = &*locals.tss as *const Tss as usize >> 22;
    pages[0][tss_pd] = unsafe { *(system_pd.offset(tss_pd as isize)) };

    let mut limit = USER_LOAD_ADDR + code.len() * 4 + 32 * PAGE_TABLE_SIZE; // 32 pages away from end of code
    limit += PAGE_TABLE_SIZE - (limit % PAGE_TABLE_SIZE);
    pages[1][0] = frame_for(code_addr) as u32 | 0x7;
    pages[0][1] = frame_for(&pages[1] as *const PageTable as usize) as u32 | 0x07; // User, RW, Present

    let cr3 = frame_for(&pages[0] as *const PageTable as usize);
    let process = Process { id: id as u32, page_tables: pages,
                            code_addr: code_addr, code_len: code_len,
                            context: Default::default() };
    processes.push_back(process);
    (limit, cr3)
}

pub fn get_current_process_mut<'r>() -> &'r mut Process {
    locals_mut().processes.front_mut().unwrap()
}

pub fn init() -> ! {
    match globals().the_code.as_ref() {
        Some(code) => {
            let (esp, cr3) = create_process(&code[..]);
            to_user_mode(USER_LOAD_ADDR, esp, cr3);
        }
        None => {
            console::puts("\r\nNo user mode (maybe no disk)\r\n");
            loop {}
        }
    }
}
