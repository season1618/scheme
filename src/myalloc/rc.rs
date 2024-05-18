use core::{
    alloc::GlobalAlloc,
    cell::Cell,
    ops::Deref,
    ptr::{self, NonNull},
};
use alloc::{
    alloc::Layout,
    borrow::Borrow,
    boxed::Box,
    fmt::{self, Display, Formatter},
};

use crate::no_std::ALLOCATOR;

#[derive(Debug)]
pub struct RcBox<T> {
    count: Cell<usize>,
    value: T,
}

#[derive(Debug)]
pub struct Rc<T> {
    ptr: NonNull<RcBox<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let rcbox = RcBox { count: Cell::new(1), value };
        Self {
            ptr: Box::leak(Box::new(rcbox)).into(),
        }
    }

    fn unbox(&self) -> &RcBox<T> {
        unsafe { self.ptr.as_ref() }
    }

    fn count(&self) -> usize {
        self.unbox().count.get()
    }

    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        ptr::addr_eq(this.ptr.as_ptr(), other.ptr.as_ptr())
    }
}

impl<T> Borrow<T> for Rc<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        self.unbox().count.set(self.count() + 1);
        Self { ptr: self.ptr }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &unsafe { self.ptr.as_ref() }.value
    }
}

impl<T: Display> Display for Rc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        self.unbox().count.set(self.count() - 1);
        if self.count() == 0 {
            unsafe {
                ptr::drop_in_place(&mut (*self.ptr.as_ptr()).value as *mut T);
                ALLOCATOR.dealloc(self.ptr.cast().as_ptr(), Layout::for_value(self.unbox()));
            }
        }
    }
}

impl<T: PartialEq> PartialEq for Rc<T> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}
