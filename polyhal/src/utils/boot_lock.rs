use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

pub struct BootLock<T: ?Sized> {
    locked: AtomicBool,
    used: AtomicU32,
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Sync for BootLock<T> {}

impl<T: ?Sized> Deref for BootLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data.get() }
    }
}

impl<T> BootLock<T> {
    pub const fn new(data: T) -> BootLock<T> {
        Self {
            locked: AtomicBool::new(false),
            used: AtomicU32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn get_mut(&self) -> BootLockGuard<T> {
        assert!(!self.locked.load(Ordering::SeqCst));
        BootLockGuard::new(self)
    }

    pub fn lock_forever(&self) {
        self.locked.store(true, Ordering::SeqCst);
    }
}

/// The Mutex Guard.
pub struct BootLockGuard<'a, T: ?Sized + 'a> {
    guard: &'a BootLock<T>,
}

impl<T: ?Sized> Deref for BootLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.guard.data.get() }
    }
}

impl<T: ?Sized> DerefMut for BootLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.guard.data.get().as_mut().unwrap() }
    }
}

impl<T: ?Sized> Drop for BootLockGuard<'_, T> {
    fn drop(&mut self) {
        self.guard.used.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<'a, T: ?Sized> BootLockGuard<'a, T> {
    pub fn new(guard: &'a BootLock<T>) -> Self {
        guard.used.fetch_add(1, Ordering::AcqRel);
        Self { guard }
    }
}
