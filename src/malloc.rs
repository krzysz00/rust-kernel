const HEAP_START: usize = 0xC0_00_00_00;
const HEAP_END: usize = 0xD0_00_00_00;

use core;
use mutex::Mutex;
use rlibc;

static HEAP_PTR_NOTEX: Mutex<usize> = mutex!(HEAP_START);

#[no_mangle]
pub extern fn rust_allocate(size: usize, align: usize) -> *mut u8 {
    let mut heap_ptr = HEAP_PTR_NOTEX.lock();
    let new_heap_ptr  = *heap_ptr & !(align - 1);

    if new_heap_ptr + size >= HEAP_END {
        core::ptr::null_mut()
    }
    else {
        *heap_ptr = new_heap_ptr + size;
        new_heap_ptr as *mut u8
    }
}

#[no_mangle]
pub extern fn rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
    let new_ptr = rust_allocate(size, align);
    if !ptr.is_null() {
        unsafe {
            rlibc::memcpy(new_ptr, ptr, old_size);
        }
    }
    ptr
}


#[allow(unused_variables)]
#[no_mangle]
pub extern fn rust_reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> usize {
    old_size
}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {}

#[allow(unused_variables)]
#[no_mangle]
pub extern fn rust_usable_size(size: usize, align: usize) -> usize {
    size
}

#[no_mangle]
pub extern fn rust_stats_print() {}
