pub use crate::generated_bumpalo::opentelemetry_::proto_::*;

use bumpalo::Bump;
use std::cell::Cell;
use std::mem::MaybeUninit;
use std::ops::Deref;

thread_local! {
    static POOL: Cell<*mut bumpalo::Bump> = const { Cell::new(std::ptr::null_mut()) };
}

struct PoolScopeGuard {
    old: *mut bumpalo::Bump,
    current: *mut bumpalo::Bump,
}

impl Drop for PoolScopeGuard {
    fn drop(&mut self) {
        POOL.with(|slot| {
            let this = slot.replace(self.old);
            debug_assert_eq!(this, self.current);
        });
    }
}

pub fn with_pool_in_scope<R>(bump: &Bump, scope: impl FnOnce() -> R) -> R {
    POOL.with(|slot| {
        let current = std::ptr::from_ref(bump) as *mut _;
        let old = slot.replace(current);
        let _guard = PoolScopeGuard { old, current };
        scope()
    })
}

fn pool() -> &'static Bump {
    POOL.with(|slot| {
        let pool = slot.get();
        assert!(!pool.is_null(), "no bump pool installed in thread-local storage");
        unsafe { &*pool }
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafeVec<T: 'static>(pub bumpalo::collections::Vec<'static, T>);

impl<T: 'static> Deref for UnsafeVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: 'static> Default for UnsafeVec<T> {
    fn default() -> Self {
        Self(bumpalo::collections::Vec::new_in(pool()))
    }
}

impl micropb::PbString for UnsafeVec<u8> {
    unsafe fn pb_set_len(&mut self, len: usize) {
        self.0.set_len(len);
    }

    fn pb_clear(&mut self) {
        self.0.clear();
    }

    fn pb_reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
        let len = self.len();
        unsafe {
            let slice = core::slice::from_raw_parts_mut(
                self.0.as_mut_ptr() as *mut MaybeUninit<u8>,
                self.0.capacity(),
            );
            slice.get_mut(len..).unwrap_or(&mut [])
        }
    }
}

impl micropb::PbBytes for UnsafeVec<u8> {}

impl<T: 'static> micropb::PbVec<T> for UnsafeVec<T> {
    fn pb_push(&mut self, elem: T) -> Result<(), ()> {
        Ok(self.0.push(elem))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafeString(pub bumpalo::collections::String<'static>);

impl Deref for UnsafeString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UnsafeString {
    fn default() -> Self {
        Self(bumpalo::collections::String::new_in(pool()))
    }
}

impl micropb::PbString for UnsafeString {
    unsafe fn pb_set_len(&mut self, len: usize) {
        self.0.as_mut_vec().set_len(len);
    }

    fn pb_clear(&mut self) {
        self.0.clear();
    }

    fn pb_reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
        let len = self.len();
        unsafe {
            let slice = core::slice::from_raw_parts_mut(
                self.0.as_mut_ptr() as *mut MaybeUninit<u8>,
                self.0.capacity(),
            );
            slice.get_mut(len..).unwrap_or(&mut [])
        }
    }
}
