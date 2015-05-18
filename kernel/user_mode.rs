use paging::{PageTable, frame_for, PAGE_TABLE_SIZE, PAGE_SIZE};
use smp::{locals, locals_mut, globals};
use mutex::Mutex;
use interrupts::{Context,Contextable};

use interrupts::apic;
use tasks;
use console;
use malloc::must_allocate;

use core::prelude::*;
use core::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use core::mem::size_of;
use alloc::boxed::Box;
use collections::{Vec,VecDeque};

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

static PROCESSES_LOCK: Mutex<Option<VecDeque<Process>>> = mutex!(None);
static PID: AtomicUsize = ATOMIC_USIZE_INIT;

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

fn create_process(code: &[u32]) {
    let locals = locals_mut();
    let mut lock = PROCESSES_LOCK.lock();
    let mut processes = lock.as_mut().unwrap();
    let id = PID.fetch_add(1, Ordering::SeqCst);

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
}

pub fn current_process<'r>() -> &'r Process {
    locals().process.as_ref().unwrap()
}

pub fn current_process_mut<'r>() -> &'r mut Process {
    locals_mut().process.as_mut().unwrap()
}

pub fn switch_tasks<T: Contextable>(ctx: &mut T) {
    let locals = locals_mut();
    let mut lock = PROCESSES_LOCK.lock();
    let mut processes = lock.as_mut().unwrap();
    let mut current_process = locals.process.take().unwrap();
    ctx.save(&mut current_process.context);
    processes.push_back(current_process);
    let new_process = processes.pop_front().unwrap();
    ctx.load(&new_process.context);
    locals.process = Some(new_process);
}

pub fn kill_current_process<T: Contextable>(ctx: &mut T) {
    let locals = locals_mut();
    let mut current_process = locals.process.take().unwrap();
    ctx.save(&mut current_process.context);
    drop(current_process);
    let mut lock = PROCESSES_LOCK.lock();
    match lock.as_mut().unwrap().pop_front() {
        Some(p) => {
            ctx.load(&p.context);
            locals.process = Some(p);
        },
        None => {
            log!("No more processes on CPU {}\r\n", apic::id());
            drop(lock);
            loop {}
        }
    }
}
pub fn init() -> ! {
    {
        let mut lock = PROCESSES_LOCK.lock();
        if lock.is_none() {
            *lock = Some(VecDeque::new());
            PID.store(1, Ordering::SeqCst);
        }
    }
    match globals().the_code.as_ref() {
        Some(code) => {
            create_process(&code[..]);
            create_process(&code[..]);
            let mut lock = PROCESSES_LOCK.lock();
            let process = lock.as_mut().unwrap().pop_front().unwrap();
            let locals = locals_mut();
            locals.process = Some(process);
            drop(lock);
            locals.process.as_ref().unwrap().context.to_user_mode();
        }
        None => {
            console::puts("\r\nNo user mode (maybe no disk)\r\n");
            loop {}
        }
    }
}
