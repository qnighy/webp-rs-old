use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut, Drop};
use std::os::raw::*;
use std::ptr::NonNull;
use std::slice;

use sys;

pub struct WebpBox<T: ?Sized> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T: ?Sized> WebpBox<T> {
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Self {
            ptr: NonNull::new_unchecked(raw),
            _marker: PhantomData,
        }
    }

    pub fn into_raw(b: Self) -> *mut T {
        let ptr = b.ptr;
        mem::forget(b);
        ptr.as_ptr()
    }

    pub fn leak<'a>(b: Self) -> &'a mut T {
        unsafe { &mut *Self::into_raw(b) }
    }
}

impl<T> WebpBox<[T]> {
    pub(crate) unsafe fn from_raw_parts(p: *mut T, len: usize) -> Self {
        Self::from_raw(slice::from_raw_parts_mut(p, len))
    }
}

impl<T: ?Sized> Deref for WebpBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for WebpBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

// unsafe impl<#[may_dangle] T: ?Sized> Drop for WebpBox<T> {
impl<T: ?Sized> Drop for WebpBox<T> {
    fn drop(&mut self) {
        unsafe {
            sys::WebPFree(self.ptr.as_ptr() as *mut c_void);
        }
    }
}

unsafe impl<T: Send + ?Sized> Send for WebpBox<T> {}
unsafe impl<T: Sync + ?Sized> Sync for WebpBox<T> {}

impl<T: ?Sized> AsRef<T> for WebpBox<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> AsMut<T> for WebpBox<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> Borrow<T> for WebpBox<T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> BorrowMut<T> for WebpBox<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: fmt::Display + ?Sized> fmt::Display for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized> fmt::Pointer for WebpBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}
