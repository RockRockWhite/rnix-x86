use core::cell::{Ref, RefCell, RefMut};

/// 用于提供单处理器下的内部可变性
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(Value: T) -> Self {
        Self {
            inner: RefCell::new(Value),
        }
    }

    /// Panic if the data has been borrowed.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }

    /// Panic if the data has been borrowed mut.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }
}
