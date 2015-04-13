// Notex - it's not a mutex, but it acts like one
use core::prelude::*;
use core::cell::UnsafeCell;
use core::ops::{Deref,DerefMut};

pub struct Notex<T> {
    pub data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Notex<T> { }
unsafe impl<T: Send> Sync for Notex<T> { }

pub struct HeldNotex<'a, T: 'a> {
    data: &'a UnsafeCell<T>,
}

impl<T> Notex<T> {
    pub fn new(t: T) -> Notex<T> {
        Notex { data: UnsafeCell::new(t) }
    }

    #[inline]
    pub fn lock(&self) -> HeldNotex<T> {
        HeldNotex { data: &self.data }
    }
}

impl<'lock, T> Deref for HeldNotex<'lock, T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.data.get() }
    }
}

impl<'lock, T> DerefMut for HeldNotex<'lock, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.data.get() }
    }
}

#[macro_export]
macro_rules! notex {
    ($val:expr) => (
        $crate::notex::Notex {
            data: ::core::cell::UnsafeCell { value: $val }
        });
    ($ty:ty, $val:expr) => (
        $crate::notex::Notex<$ty> {
            data: ::core::cell::UnsafeCell<$ty> { value: $val }
        });
}
