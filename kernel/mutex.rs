use core::cell::UnsafeCell;
use core::ops::{Deref,DerefMut};
use core::sync::atomic::{fence, Ordering, AtomicBool};

pub struct Mutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> { }
unsafe impl<T: Send> Sync for Mutex<T> { }

pub struct HeldMutex<'a, T: 'a> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    // TODO: Make this a const fn and remove the macro when 1.2 hits
    pub const fn new(t: T) -> Mutex<T> {
        Mutex { lock: AtomicBool::new(false), data: UnsafeCell::new(t) }
    }

    pub fn lock(&self) -> HeldMutex<T> {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) == true {};
        fence(Ordering::Acquire);
        HeldMutex { mutex: self }
    }

    fn unlock(&self) {
        fence(Ordering::Release);
        self.lock.store(false, Ordering::Release);
    }
}

impl<'lock, T> Deref for HeldMutex<'lock, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'lock, T> DerefMut for HeldMutex<'lock, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<'lock, T> Drop for HeldMutex<'lock, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
