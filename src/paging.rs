use machine;
use notex::Notex;

const PAGE_TABLE_ENTRIES: usize = 512;

type PageTable = [u64; PAGE_TABLE_ENTRIES];

static NEXT_FRAME_NOTEX: Notex<u64> = notex!(0x200);

fn page_table_for(vaddr: u64, next_frame: &mut u64) -> &'static mut PageTable {
    let pd_index = vaddr >> 22;
    unsafe {
        // PD[1023] -> pd_addr. pd_addr[1023] -> pd_addr
        let pd = 0xFFFFF_000 as *mut PageTable;
        if (*pd)[pd_index as usize] & 1 == 0 {
            (*pd)[pd_index as usize] = (*next_frame << 12) | 0b11;
            *next_frame += 1;
        }
        &mut *((0xFFC00_000 + 0x1_000 * pd_index) as *mut PageTable)
    }
}

#[allow(unused_assignments)]
pub fn make_present(addr: u64) {
    let mut next_frame = NEXT_FRAME_NOTEX.lock();
    let page_number = (addr >> 12) & 0x3ff;
    let pt = page_table_for(addr, &mut *next_frame);
    pt[page_number as usize] = *next_frame << 12 | 0b11;
    *next_frame += 1;
}
