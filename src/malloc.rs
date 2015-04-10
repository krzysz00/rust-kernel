const HEAP_START: usize = 0x00100000;
const HEAP_END: usize = 0x00EFFFFF;

static mut heap_ptr: usize = HEAP_START;

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
