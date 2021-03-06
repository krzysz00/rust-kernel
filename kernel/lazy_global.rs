use core::cell::UnsafeCell;

pub struct LazyGlobal<T> {
    data: UnsafeCell<Option<T>>,
}

unsafe impl<T: Send> Send for LazyGlobal<T> { }
unsafe impl<T: Send> Sync for LazyGlobal<T> { }

impl<T> LazyGlobal<T> {
    pub const fn new() -> LazyGlobal<T> {
        LazyGlobal { data: UnsafeCell::new(None) }
    }

    // You must call this before using the global
    pub unsafe fn init(&self, val: T) {
        *self.data.get() = Some(val);
    }

    pub unsafe fn get<'a>(&'a self) -> &'a T {
        match *self.data.get() {
            Some(ref val) => val,
            None => panic!("Lazy global not initialized")
        }
    }

    pub unsafe fn get_mut<'a>(&'a self) -> &'a mut T {
        match *self.data.get() {
            Some(ref mut val) => val,
            None => panic!("Lazy global not initialized")
        }
    }
}
