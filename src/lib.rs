#![allow(nonstandard_style, unused, irrefutable_let_patterns)]

#[cfg(feature = "use_std")]
mod generated_std;

#[cfg(feature = "use_std")]
pub mod std {
    pub use crate::generated_std::opentelemetry_::proto_::*;
}

#[cfg(feature = "use_std")]
mod generated_bumpalo;

#[cfg(feature = "use_bumpalo")]
pub mod bumpalo {
    pub use crate::generated_bumpalo::opentelemetry_::proto_::*;

    use bumpalo::Bump;
    use std::mem::MaybeUninit;
    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::ops::Deref;
    use std::cell::Cell;

    thread_local! {
        static POOL: Cell<*mut bumpalo::Bump> = const { Cell::new(std::ptr::null_mut()) };
    }

    pub fn set_pool_in_scope(bump: &Bump, scope: impl Fn()){
        POOL.with(|ptr| {
            let old = ptr.replace(std::ptr::from_ref(bump) as *mut _);
            scope();
            let this = ptr.replace(old);
            debug_assert_eq!(this, std::ptr::from_ref(bump) as *mut _);
        })
    }

    fn pool() -> &'static Bump {
        let pool = POOL.get();
        debug_assert!(!pool.is_null());
        unsafe { &mut*pool }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UnsafeVec<T: 'static>(pub bumpalo::collections::Vec<'static, T>);

    impl<T: 'static> Deref for UnsafeVec<T> {
        type Target = [T];
        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl<T: 'static> Default for UnsafeVec<T> {
        fn default() -> Self { Self(bumpalo::collections::Vec::new_in(pool())) }
    }

    impl<T: 'static> micropb::PbContainer for UnsafeVec<T> {
        unsafe fn pb_set_len(&mut self, len: usize) { self.0.set_len(len); }
        fn pb_clear(&mut self) { self.0.clear(); }
        fn pb_reserve(&mut self, additional: usize) { self.0.reserve(additional) }
    }

    impl<T: 'static> micropb::PbVec<T> for UnsafeVec<T> {
        // TODO handle error case? we dont have try_push()
        fn pb_push(&mut self, elem: T) -> Result<(), ()> { Ok(self.0.push(elem)) }

        fn pb_spare_cap(&mut self) -> &mut [std::mem::MaybeUninit<T>] {
            // TODO does bump::Vec offer a better method to get spare cap?

            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut MaybeUninit<T>, self.0.capacity())
            };
            slice.get_mut(len..).unwrap_or(&mut [])
        }
    
        fn pb_from_slice(s: &[T]) -> Result<Self, ()>
            where T: Copy
        {
            let mut vec = bumpalo::collections::Vec::new_in(pool());
            vec.extend_from_slice_copy(s);
            Ok(Self(vec))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct UnsafeString(pub bumpalo::collections::String<'static>);

    impl Deref for UnsafeString {
        type Target = str;
        fn deref(&self) -> &Self::Target { &self.0 }
    }

    impl Default for UnsafeString {
        fn default() -> Self { Self(bumpalo::collections::String::new_in(pool())) }
    }

    impl micropb::PbContainer for UnsafeString {
        unsafe fn pb_set_len(&mut self, len: usize) { self.0.as_mut_vec().set_len(len); }
        fn pb_clear(&mut self) { self.0.clear(); }
        fn pb_reserve(&mut self, additional: usize) { self.0.reserve(additional) }
    }

    impl micropb::PbString for UnsafeString {
        fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
            // TODO does bump::Vec offer a better method to get spare cap?

            let len = self.len();
            // SAFETY: Underlying storage is static array of size N, so it's safe to create a slice
            // of N values
            let slice = unsafe {
                core::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut MaybeUninit<u8>, self.0.capacity())
            };
            slice.get_mut(len..).unwrap_or(&mut [])
        }
    
        fn pb_from_str(s: &str) -> Result<Self, ()> {
            let str = bumpalo::collections::String::from_str_in(s, pool());
            Ok(Self(str))
        }
    }
}