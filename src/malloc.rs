const HEAP_START: usize = 0x00100000;
const HEAP_END: usize = 0x00EFFFFF;

static mut heap_ptr: usize = HEAP_START;

extern {
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
}

#[no_mangle]
pub unsafe extern fn rust_allocate(size: usize, align: usize) -> *mut u8 {
    let mut new_heap_ptr = heap_ptr;
    if heap_ptr % align != 0 {
        new_heap_ptr += (align - (heap_ptr % align));
    }

    if new_heap_ptr + size >= HEAP_END {
        0 as *mut u8
    }
    else {
        heap_ptr = new_heap_ptr + size;
        new_heap_ptr as *mut u8
    }
}

#[no_mangle]
pub unsafe extern fn rust_reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
    let new_ptr = rust_allocate(size, align);
    if !ptr.is_null() {
        memcpy(ptr, new_ptr, old_size);
    }
    ptr
}


#[allow(unused_varibles)]
#[no_mangle]
pub extern fn rust_reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> usize {
    old_size
}

#[allow(unused_varibles)]
#[no_mangle]
pub extern fn rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {}

#[allow(unused_varibles)]
#[no_mangle]
pub extern fn rust_usable_size(size: usize, align: usize) -> usize {
    size
}

#[no_mangle]
pub extern fn rust_stats_print() {}
