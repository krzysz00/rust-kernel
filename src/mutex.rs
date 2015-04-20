// Notex - it's not a mutex, but it acts like one
use core::prelude::*;
use core::cell::UnsafeCell;
use core::ops::{Deref,DerefMut};
use core::atomic::{fence, Ordering, AtomicBool};

pub struct Mutex<T> {
    pub lock: AtomicBool,
    pub data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> { }
unsafe impl<T: Send> Sync for Mutex<T> { }

pub struct HeldMutex<'a, T: 'a> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Mutex<T> {
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

#[macro_export]
macro_rules! mutex {
    ($val:expr) => (
        $crate::mutex::Mutex {
            lock: ::core::atomic::ATOMIC_BOOL_INIT,
            data: ::core::cell::UnsafeCell { value: $val }
        });
    ($ty:ty, $val:expr) => (
        $crate::mutex::Mutex<$ty> {
            lock: core::atomic::ATOMIC_BOOL_INIT,
            data: ::core::cell::UnsafeCell<$ty> { value: $val }
        });
}
