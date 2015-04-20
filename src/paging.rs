use machine;
use mutex::Mutex;
use smp;

const PAGE_TABLE_ENTRIES: usize = 512;

type PageTable = [u64; PAGE_TABLE_ENTRIES];

static NEXT_FRAME_MUTEX: Mutex<u64> = mutex!(0x200);

const MAPPING_BASE: u64 = (511 << 39) | (0xffff << 48);
const PML4_MAPPING: u64 = 0xffff_ffff_ffff_f000;

const PRESENT_RW: u64 = 0b11;

const NINE_BITS: u64 = 0x1ff;
const NINE_BITS_USIZE: usize = 0x1ff;

#[inline]
fn address_of_table(pdp: u64, pd: u64, pt: u64) -> u64 {
    MAPPING_BASE | pdp << 30 | pd << 21 | pt << 12
}

fn ensure_pdp(vaddr: u64, next_frame: &mut u64) -> &'static mut PageTable {
    let pml4_index = (vaddr >> 39) & NINE_BITS;
    unsafe {
        let pml4 = PML4_MAPPING as *mut PageTable;
        if (*pml4)[pml4_index as usize] & 1 == 0 {
            (*pml4)[pml4_index as usize] = (*next_frame << 12) | PRESENT_RW;
            *next_frame += 1;
        }
        &mut *(address_of_table(511, 511, pml4_index) as *mut PageTable)
    }
}

fn ensure_pd(vaddr: u64, next_frame: &mut u64) -> &'static mut PageTable {
    let pml4_index = (vaddr >> 39) & NINE_BITS;
    let pdp_index = (vaddr >> 30) & NINE_BITS;
    unsafe {
        let pdp = ensure_pdp(vaddr, next_frame);
        if pdp[pdp_index as usize] & 1 == 0 {
            pdp[pdp_index as usize] = (*next_frame << 12) | PRESENT_RW;
            *next_frame += 1;
        }
        &mut *(address_of_table(511, pml4_index, pdp_index) as *mut PageTable)
    }
}

fn ensure_pt(vaddr: u64, next_frame: &mut u64) -> &'static mut PageTable {
    let pml4_index = (vaddr >> 39) & NINE_BITS;
    let pdp_index = (vaddr >> 30) & NINE_BITS;
    let pd_index = (vaddr >> 21) & NINE_BITS;
    unsafe {
        let pd = ensure_pd(vaddr, next_frame);
        if pd[pd_index as usize] & 1 == 0 {
            pd[pd_index as usize] = (*next_frame << 12) | PRESENT_RW;
            *next_frame += 1;
        }
        &mut *(address_of_table(pml4_index, pdp_index, pd_index) as *mut PageTable)
    }
}

pub fn identity_map(addr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let frame_num = addr >> 12;
    let page_index = frame_num & NINE_BITS_USIZE;
    let pt = ensure_pt(addr as u64, &mut *next_frame);
    pt[page_index] = (frame_num as u64) << 12 | PRESENT_RW;
}

pub fn forget(addr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let page_index = (addr >> 12) & NINE_BITS_USIZE;
    let pt = ensure_pt(addr as u64, &mut *next_frame);
    pt[page_index] &= !1;
    machine::invlpg(addr as u64)
}

#[allow(unused_assignments)]
pub fn make_present(vaddr: usize) {
    let mut next_frame = NEXT_FRAME_MUTEX.lock();
    let pt_index = (vaddr >> 12) & NINE_BITS_USIZE;
    let pt = ensure_pt(vaddr as u64, &mut next_frame);
    pt[pt_index] = *next_frame << 12 | 0b11;
    *next_frame += 1;
}

pub fn init() {
    smp::SMP_CR3.store(0x1000, ::core::atomic::Ordering::SeqCst);
}
