use machine;
use notex::Notex;

const PAGE_TABLE_ENTRIES: usize = 1024;
const PAGE_TABLE_SIZE: usize = 4096;

type PageTable = [u32; PAGE_TABLE_ENTRIES];

// Overallocate. We'll loop through it to get a page-aligned piece of memory
static INITIAL_TABLE_NOTEX: Notex<[u32; PAGE_TABLE_ENTRIES * 3]> = notex!([0; PAGE_TABLE_ENTRIES * 3]);

static NEXT_FRAME_NOTEX: Notex<u32> = notex!(0x100);

fn page_table_for(vaddr: u32, next_frame: &mut u32) -> &'static mut PageTable {
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
pub fn make_present(addr: u32) {
    let mut next_frame = *NEXT_FRAME_NOTEX.lock();
    let page_number = (addr >> 12) & 0x3ff;
    let pt = page_table_for(addr, &mut next_frame);
    pt[page_number as usize] = next_frame << 12 | 0b11;
    next_frame += 1;
}

pub fn init() {
    unsafe {
        let initial_table_area = *INITIAL_TABLE_NOTEX.lock();
        let unalligned_pd_addr = initial_table_area[0] as *const u32 as usize;
        let alligned_pd_addr = unalligned_pd_addr +
            (PAGE_TABLE_SIZE -
             (unalligned_pd_addr % PAGE_TABLE_SIZE));
        let pd = alligned_pd_addr as *mut PageTable;

        let first_pt: *mut PageTable = pd.offset(1);
        (*pd)[0] = (first_pt as u32) | 0b11; // Read-write, present

        for frame_num in 0..0x100 {
            (*first_pt)[frame_num as usize] = (frame_num << 12) | 0b11;
        }

        (*pd)[1023] = (alligned_pd_addr as u32) | 0b11; // The "recursive paging trick"
        machine::enable_paging(alligned_pd_addr as *const u32);
    }
}
