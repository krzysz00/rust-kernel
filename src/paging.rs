use machine;
use mutex::Mutex;
use smp;

const PAGE_TABLE_ENTRIES: usize = 1024;
const PRESENT_RW: u32 = 0b11;

type PageTable = [u32; PAGE_TABLE_ENTRIES];

// Overallocate. We'll loop through it to get a page-aligned piece of memory
// static INITIAL_TABLE_NOTEX: Notex<[u32; PAGE_TABLE_ENTRIES * 3]> = notex!([0; PAGE_TABLE_ENTRIES * 3]);

static NEXT_FRAME_MUTEX: Mutex<u32> = mutex!(0x100);

fn page_table_for(vaddr: u32, next_frame: &mut u32) -> &'static mut PageTable {
    let pd_index = vaddr >> 22;
    unsafe {
        // PD[1023] -> pd_addr. pd_addr[1023] -> pd_addr
        let pd = 0xFFFFF_000 as *mut PageTable;
        if (*pd)[pd_index as usize] & 1 == 0 {
            (*pd)[pd_index as usize] = (*next_frame << 12) | PRESENT_RW;
            *next_frame += 1;
        }
        &mut *((0xFFC00_000 + 0x1_000 * pd_index) as *mut PageTable)
    }
}

pub fn identity_map(addr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let frame_num = addr >> 12;
    let page_index = frame_num & 0x3ff;
    let pt = page_table_for(addr as u32, &mut *next_frame);
    pt[page_index] = (frame_num as u32) << 12 | PRESENT_RW;
}

pub fn forget(addr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let page_index = (addr >> 12) & 0x3ff;
    let pt = page_table_for(addr as u32, &mut *next_frame);
    pt[page_index] &= !1;
    machine::invlpg(addr as u32)
}

pub fn give_to_user(addr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let pd_index = addr >> 22;
    let page_index = (addr >> 12) & 0x3ff;

    let pd = unsafe {
        &mut*(0xFFFFF_000 as *mut PageTable)
    };
    let pt = page_table_for(addr as u32, &mut *next_frame);
    pd[page_index] |= 0x5;
    pt[page_index] |= 0x5;
    machine::invlpg(addr as u32)
}

#[allow(unused_assignments)]
pub fn make_present(addr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let page_number = (addr >> 12) & 0x3ff;
    let pt = page_table_for(addr as u32, &mut *next_frame);
    pt[page_number] = *next_frame << 12 | PRESENT_RW;
    *next_frame += 1;
}

pub fn init() {
    unsafe {
        let alligned_pd_addr = 0x1000;
        let pd = alligned_pd_addr as *mut PageTable;

        let first_pt: *mut PageTable = pd.offset(1);
        (*pd)[0] = (first_pt as u32) | PRESENT_RW; // Read-write, present

        for frame_num in 0..0x100 {
            (*first_pt)[frame_num as usize] = (frame_num << 12) | PRESENT_RW;
        }

        (*pd)[1023] = (alligned_pd_addr as u32) | PRESENT_RW; // The "recursive paging trick"
        smp::SMP_CR3.store(alligned_pd_addr, ::core::atomic::Ordering::SeqCst);
        machine::enable_paging(alligned_pd_addr as *const u32);
    }
}
