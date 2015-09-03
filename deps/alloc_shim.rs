#![feature(allocator,no_std)]
#![no_std]
#![allocator]

#[allow(improper_ctypes)]
extern {
    fn rust_allocate(size: usize, align: usize) -> *mut u8;
    fn rust_deallocate(ptr: *mut u8, old_size: usize, align: usize);
    fn rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                         align: usize) -> *mut u8;
    fn rust_reallocate_inplace(ptr: *mut u8, old_size: usize, size: usize,
                               align: usize) -> usize;
    fn rust_usable_size(size: usize, align: usize) -> usize;
}

#[no_mangle]
#[inline(always)]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe { rust_allocate(size, align) }
}

#[no_mangle]
#[inline(always)]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { rust_deallocate(ptr, old_size, align) }
}

#[no_mangle]
#[inline(always)]
pub extern fn __rust_reallocate(ptr: *mut u8, old_size: usize, size: usize,
                                align: usize) -> *mut u8 {
    unsafe { rust_reallocate(ptr, old_size, size, align) }
}

#[no_mangle]
#[inline(always)]
pub extern fn __rust_reallocate_inplace(ptr: *mut u8, old_size: usize,
                                        size: usize, align: usize) -> usize {
    unsafe { rust_reallocate_inplace(ptr, old_size, size, align) }
}

#[no_mangle]
#[inline(always)]
pub extern fn __rust_usable_size(size: usize, align: usize) -> usize {
    unsafe { rust_usable_size(size, align) }
}
