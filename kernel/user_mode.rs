use machine::to_user_mode;
use paging::{PageTable, frame_for, PAGE_TABLE_SIZE, PAGE_SIZE};
use smp::{locals_mut, globals};
use interrupts::{Context,Contextable};

use interrupts::apic;
use tasks;
use console;
use malloc::must_allocate;

use core::prelude::*;
use core::mem::size_of;
use alloc::boxed::Box;
use collections::Vec;

pub const USER_LOAD_ADDR: usize = 0x400_000;

pub type Page = Box<PageTable>;

fn new_user_page() -> Page {
    unsafe {
        Box::from_raw(must_allocate(size_of::<PageTable>(), PAGE_SIZE) as *mut PageTable)
    }
}

pub struct Process {
    pub id: u32,
    pub pd: Page,
    pub code_addr: usize,
    pub code_len: usize,
    pub pages: Vec<Page>,
    pub context: Context,
}

impl Process {
    // Adds a page to the process, returning its frame and index in the pages array
    pub fn add_page(&mut self) -> (usize, u32) {
        let mut page = new_user_page();
        page[0] = 0;
        let frame = frame_for(&page[0] as *const u32 as usize) as u32;
        let len = self.pages.len();
        self.pages.push(page);
        (len, frame)
    }
}

fn create_process(code: &[u32]) -> (usize, usize) {
    let locals = locals_mut();
    let ref mut processes = locals.processes;
    let id = processes.len() + 1;

    let code_addr = &code[0] as *const u32 as usize;
    let code_len = code.len();
    let mut pd = new_user_page();
    let system_pd = 0xFF_FF_F000 as *const u32;
    pd[0] = unsafe { *(system_pd) };
    let tss_pd = &locals.tss as *const tasks::Tss as usize >> 22;
    pd[tss_pd] = unsafe { *(system_pd.offset(tss_pd as isize)) };

    let mut limit = USER_LOAD_ADDR + code.len() * 4 + 32 * PAGE_TABLE_SIZE; // 32 pages away from end of code
    limit += PAGE_TABLE_SIZE - (limit % PAGE_TABLE_SIZE);
    let cr3 = frame_for(&*pd as *const PageTable as usize);

    let mut process = Process { id: id as u32, pd: pd, pages: Vec::new(),
                                code_addr: code_addr, code_len: code_len,
                                context: Context::new(USER_LOAD_ADDR, limit, cr3) };

    let (idx, frame) = process.add_page();
    process.pages[idx][0] = frame_for(code_addr) as u32 | 0x7;
    process.pd[1] = frame | 0x07; // User, RW, Present
    processes.push_back(process);
    (limit, cr3)
}

pub fn get_current_process_mut<'r>() -> &'r mut Process {
    locals_mut().processes.front_mut().unwrap()
}

pub fn switch_tasks<T: Contextable>(ctx: &mut T) {
    let ref mut processes = locals_mut().processes;
    let mut current_process = processes.pop_front().unwrap();
    ctx.save(&mut current_process.context);
    processes.push_back(current_process);
    let new_process = processes.front().unwrap();
    ctx.load(&new_process.context);
}

pub fn kill_current_process<T: Contextable>(ctx: &mut T) {
    let ref mut processes = locals_mut().processes;
    let mut current_process = processes.pop_front().unwrap();
    ctx.save(&mut current_process.context);
    drop(current_process);
    match processes.front_mut() {
        Some(p) => ctx.load(&p.context),
        None => {
            log!("No more processes on CPU {}\r\n", apic::id());
            loop {}
        }
    }
}
pub fn init() -> ! {
    match globals().the_code.as_ref() {
        Some(code) => {
            let (esp, cr3) = create_process(&code[..]);
            create_process(&code[..]);
            to_user_mode(USER_LOAD_ADDR, esp, cr3);
        }
        None => {
            console::puts("\r\nNo user mode (maybe no disk)\r\n");
            loop {}
        }
    }
}
