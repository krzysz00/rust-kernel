use notex::Notex;

const PAGE_TABLE_ENTRIES: usize = 512;

type PageTable = [u64; PAGE_TABLE_ENTRIES];

static NEXT_FRAME_NOTEX: Notex<u64> = notex!(0x200);

const PT_MAP_BASE: u64 = 0xFFFF_FF80_0000_0000;
const PD_MAP_BASE: u64 = 0xFFFF_FFFF_C000_0000;
const PDP_MAP_BASE: u64 = 0xFFFF_FFFF_FFE0_0000;
const PML_MAP_BASE: u64 = 0xFFFF_FFFF_FFFF_F000;

const LEVEL3_OFFSET: u64 = 0x4000_0000;
const LEVEL2_OFFSET: u64 = 0x20_0000;
const LEVEL1_OFFSET: u64 = 0x1000;

const PRESENT_RW: u64 = 0b11;

const NINE_BITS: u64 = 0x1ff;

fn ensure_pdp(vaddr: u64, next_frame: &mut u64) -> &'static mut PageTable {
    let pml4_index = (vaddr >> 39) & NINE_BITS;
    unsafe {
        let pml4 = PML_MAP_BASE as *mut PageTable;
        if (*pml4)[pml4_index as usize] & 1 == 0 {
            (*pml4)[pml4_index as usize] = (*next_frame << 12) | PRESENT_RW;
            *next_frame += 1;
        }
        &mut *((PDP_MAP_BASE + LEVEL1_OFFSET * pml4_index) as *mut PageTable)
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
        &mut *((PD_MAP_BASE + LEVEL2_OFFSET * pml4_index +
               LEVEL1_OFFSET * pdp_index) as *mut PageTable)
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
        &mut *((PT_MAP_BASE + LEVEL3_OFFSET * pml4_index +
                LEVEL2_OFFSET * pdp_index + LEVEL3_OFFSET * pd_index)
               as *mut PageTable)
    }
}

#[allow(unused_assignments)]
pub fn make_present(vaddr: u64) {
    let mut next_frame = NEXT_FRAME_NOTEX.lock();
    let pt_index = (vaddr >> 12) & NINE_BITS;
    let pt = ensure_pt(vaddr, &mut next_frame);
    pt[pt_index as usize] = *next_frame << 12 | 0b11;
    *next_frame += 1;
}
