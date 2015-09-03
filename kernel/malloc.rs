const HEAP_START: usize = 0xC0_00_00_00;
// const HEAP_END: usize = 0xD0_00_00_00;

use core::cmp;
use core::ptr::null_mut;
use core::option::Option::{self, Some, None};
use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
use core::intrinsics::copy_nonoverlapping;

use mutex::Mutex;

// This mutex does not guard anything.
// However, all top-level functions that manipulate the heap must take it
static HEAP_MUTEX: Mutex<()> = Mutex::new(());

const MAX_ORDER: u8 = 22;
const BLOCK_BITS: u8 = 5;

const USED_MAGIC: [u8; 4] = [0; 4];
const FREE_MAGIC: [u8; 4] = ['F' as u8, 'R' as u8, 'E' as u8, 'E' as u8];

#[repr(packed)]
struct Block {
    pub order: u8,
    pub magic: [u8; 4],
    prev: *mut Block,
    next: *mut Block,
}

// Guarded by the big fat heap lock
static mut free_lists: [*mut Block; (MAX_ORDER + 1) as usize] =
    [0 as *mut Block; (MAX_ORDER + 1) as usize];

static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

unsafe fn to_block<'r>(ptr: *mut u8) -> &'r mut Block {
    &mut *(ptr.offset(-1) as *mut Block)
}

impl Block {
    fn to_pointer(&mut self) -> *mut u8 {
        &mut self.magic[0] as *mut u8
    }

    fn to_const_pointer(&self) -> *const u8 {
        &self.magic[0] as *const u8
    }

    fn buddy<'r>(&self) -> &'r mut Block {
        unsafe {
            let addr = self.to_const_pointer() as usize;
            let buddy_addr = addr ^ ((1 << (self.order + BLOCK_BITS)) as usize);
            to_block(buddy_addr as *mut u8)
        }
    }

    unsafe fn free_insert(&mut self) {
        let mut prev = null_mut();
        let mut next = free_lists[self.order as usize];
        let self_ptr = self as *mut Block;
        let addr = self_ptr as usize;
        while !next.is_null() && (next as usize) < addr {
            prev = next;
            next = (*next).next;
        }
        self.prev = prev;
        self.next = next;
        if !next.is_null() {
            (*next).prev = self_ptr;
        }
        if prev.is_null() {
            free_lists[self.order as usize] = self_ptr;
        }
        else {
            (*prev).next = self_ptr
        }
    }

    unsafe fn free_remove(&mut self) {
        if !self.prev.is_null() {
            (*self.prev).next = self.next;
        }
        if !self.next.is_null() {
            (*self.next).prev = self.prev;
        }
        if free_lists[self.order as usize] == self as *mut Block {
            free_lists[self.order as usize] = self.next;
        }
    }

    fn make_used(&mut self) {
        self.magic = USED_MAGIC;
        unsafe { self.free_remove() ;}
        self.prev = null_mut();
        self.next = null_mut();
    }

    fn free(&mut self) {
        self.magic = FREE_MAGIC;
        unsafe { self.free_insert(); }
    }

    fn split<'r>(&mut self) -> &'r mut Block {
        self.make_used();
        self.order -= 1;
        self.free();
        let ret = self.buddy();
        ret.order = self.order;
        ret.free();
        ret
    }

    fn maybe_merge<'r>(&mut self){
        let buddy = self.buddy();
        if self.magic != FREE_MAGIC || buddy.magic != FREE_MAGIC || self.order == MAX_ORDER {
            return;
        }
        let is_self_smaller = (self as *mut Block as usize) < (buddy as *mut Block as usize);
        let (smaller, larger) = if is_self_smaller { (self, buddy) } else { (buddy, self) };
        if smaller.next != larger || larger.prev != smaller {
            return // Not actually free blocks, unless weird things happened
        }
        smaller.make_used();
        larger.make_used();
        larger.order = 0;
        smaller.order += 1;
        smaller.free();
        smaller.maybe_merge(); // Get as much merging in as we can
    }
}

unsafe fn init() {
    if INITIALIZED.compare_and_swap(false, true, Ordering::SeqCst) == false {
        let top_addr = HEAP_START - 1;
        let top = &mut *(top_addr as *mut u8 as *mut Block);
        top.order = MAX_ORDER;
        top.free();

        let buddy = top.buddy();
        buddy.order = MAX_ORDER;
        buddy.free();
    }
}

fn order_of(size: usize) -> u8 {
    let rounded = size.next_power_of_two();
    let log = rounded.trailing_zeros() as u8;
    cmp::max(log, BLOCK_BITS) - BLOCK_BITS
}

#[inline]
fn block_size(order: u8) -> usize {
    (1 << (order + BLOCK_BITS)) - 1
}

fn find_block_of_order<'r>(order: u8) -> Option<&'r mut Block> {
    if order > MAX_ORDER {
        return None
    }
    unsafe {
        if !INITIALIZED.load(Ordering::Relaxed) {
            init();
        }
        let maybe_head = free_lists[order as usize];
        if !maybe_head.is_null() {
            Some(&mut *maybe_head)
        }
        else {
            find_block_of_order(order + 1).map(|block| { block.split(); block })
        }
    }
}

#[inline]
fn to_order(size: usize, align: usize) -> u8 {
    cmp::max(order_of(size), order_of(align))
}

#[no_mangle]
pub unsafe extern fn rust_allocate(size: usize, align: usize) -> *mut u8 {
    let _lock = HEAP_MUTEX.lock();
    let order = to_order(size + 1, align);
    match find_block_of_order(order) {
        Some(block) => {
            block.make_used();
            block.to_pointer()
        }
        None => null_mut()
    }
}

#[inline]
pub unsafe fn malloc(size: usize) -> *mut u8 {
    rust_allocate(size, 1)
}

pub fn must_allocate(size: usize, align: usize) -> *mut u8 {
    let ptr = unsafe { rust_allocate(size, align) };
    if !ptr.is_null() {
        ptr
    }
    else {
        panic!("Out of memory");
    }
}

#[inline]
#[no_mangle]
pub unsafe extern fn rust_deallocate(ptr: *mut u8,
                                     _old_size: usize, _align: usize) {
    free(ptr)
}

pub unsafe fn free(ptr: *mut u8) {
    let _lock = HEAP_MUTEX.lock();
    let block = to_block(ptr);
    block.free();
    block.maybe_merge();
}

#[inline]
pub unsafe fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    rust_reallocate(ptr, 1, size, 1)
}

#[no_mangle]
pub unsafe extern fn rust_reallocate(ptr: *mut u8, _old_size: usize,
                                     size: usize, align: usize) -> *mut u8 {
    let _lock = HEAP_MUTEX.lock();
    let block = to_block(ptr);
    let new_order = to_order(size + 1, align);
    if block.order >= new_order {
        ptr
    }
    else {
        match find_block_of_order(new_order) {
            Some(new_block) => {
                new_block.make_used();
                let ret = new_block.to_pointer();
                copy_nonoverlapping(ptr, ret, block_size(block.order));
                block.free();
                block.maybe_merge();
                ret
            }
            None => null_mut()
        }
    }
}

#[no_mangle]
pub unsafe extern fn rust_reallocate_inplace(ptr: *mut u8, old_size: usize,
                                             size: usize, align: usize) -> usize {
    let _lock = HEAP_MUTEX.lock();
    let block = to_block(ptr);
    let new_order = to_order(size + 1, align);
    if block.order >= new_order {
        block_size(block.order)
    }
    else {
        rust_usable_size(old_size, align)
    }
}

#[no_mangle]
pub extern fn rust_usable_size(size: usize, align: usize) -> usize {
    let order = to_order(size + 1, align);
    block_size(order)
}

#[no_mangle]
pub extern fn rust_stats_print() {}
