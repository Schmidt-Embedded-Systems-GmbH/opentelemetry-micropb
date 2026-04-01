//! Append-only region allocator and container adapters for generated `micropb` code.
//!
//! This backend uses fixed-size pooled segments with a hard upper bound on total memory.
//! New areas check out segments from an [`AreaPool`], and reset/drop return spare segments to the
//! pool for reuse.
//!
//! Note the following:
//! - There is no `map` container implementation, because we do not needthemfor opentelemetry-proto.
//! - Allocation is bump-only inside each segment. Individual vectors/strings do not reclaim old
//!   capacity when grown.
//! - [`Area::reset`] is still unsafe because callers must guarantee no live values still point into
//!   the area.
//! - Cross-thread use still requires the caller to coordinate which root area is installed in TLS.

use core::alloc::Layout;
use core::cell::Cell;
use core::fmt;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::ptr::{self, NonNull};
use std::boxed::Box;
use std::vec::Vec;

#[cfg(not(feature = "use_spinlock"))]
use std::sync::Mutex;
#[cfg(feature = "use_spinlock")]
use spin::Mutex;

#[cfg(not(feature = "use_spinlock"))]
fn lock_mutex<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poison| poison.into_inner())
}

#[cfg(feature = "use_spinlock")]
fn lock_mutex<T>(mutex: &Mutex<T>) -> spin::mutex::MutexGuard<'_, T> {
    mutex.lock()
}

thread_local! {
    static CURRENT_AREA: Cell<*const Area> = const { Cell::new(ptr::null()) };
}

struct AreaScopeGuard {
    old: *const Area,
    current: *const Area,
}

impl Drop for AreaScopeGuard {
    fn drop(&mut self) {
        CURRENT_AREA.with(|slot| {
            let this = slot.replace(self.old);
            debug_assert_eq!(this, self.current);
        });
    }
}

/// Executes `scope` with `area` installed as the current thread-local allocation region.
pub fn with_area_in_scope<R>(area: &Area, scope: impl FnOnce() -> R) -> R {
    CURRENT_AREA.with(|slot| {
        let current = ptr::from_ref(area);
        let old = slot.replace(current);
        let _guard = AreaScopeGuard { old, current };
        scope()
    })
}

fn current_area_ptr() -> *const Area {
    CURRENT_AREA.with(|slot| {
        let ptr = slot.get();
        assert!(!ptr.is_null(), "no area installed in thread-local storage");
        ptr
    })
}

/// Pool of fixed-size segments used to back [`Area`] allocations.
pub struct AreaPool {
    segment_size: usize,
    max_segments: usize,
    inner: Mutex<PoolInner>,
}

struct PoolInner {
    segments: Vec<Box<[MaybeUninit<u8>]>>,
    free: Vec<usize>,
}

impl AreaPool {
    /// Creates a new pool with fixed-size segments and a hard cap on segment count.
    pub fn new(segment_size: usize, max_segments: usize) -> Self {
        assert!(segment_size > 0, "segment_size must be > 0");
        assert!(max_segments > 0, "max_segments must be > 0");
        Self {
            segment_size,
            max_segments,
            inner: Mutex::new(PoolInner {
                segments: Vec::new(),
                free: Vec::new(),
            }),
        }
    }

    /// Size of each segment in bytes.
    pub fn segment_size(&self) -> usize {
        self.segment_size
    }

    /// Maximum number of segments the pool will ever allocate.
    pub fn max_segments(&self) -> usize {
        self.max_segments
    }

    /// Maximum number of bytes the pool can back in total.
    pub fn max_bytes(&self) -> usize {
        self.segment_size * self.max_segments
    }

    /// Checks out a fresh area with one segment from the pool.
    pub fn checkout(&self) -> Option<Area> {
        let first = self.acquire_segment()?;
        Some(Area {
            pool: self as *const _,
            segments: Mutex::new(vec![first]),
        })
    }

    fn acquire_segment(&self) -> Option<AreaSegment> {
        let mut inner = lock_mutex(&self.inner);
        let index = if let Some(index) = inner.free.pop() {
            index
        } else {
            if inner.segments.len() >= self.max_segments {
                return None;
            }
            inner.segments.push(vec![MaybeUninit::<u8>::uninit(); self.segment_size].into_boxed_slice());
            inner.segments.len() - 1
        };

        let segment = &mut inner.segments[index];
        let base = NonNull::new(segment.as_mut_ptr().cast::<u8>())
            .unwrap_or_else(NonNull::dangling);
        Some(AreaSegment {
            index,
            base,
            len: self.segment_size,
            cursor: 0,
        })
    }

    fn release_segments(&self, indices: impl IntoIterator<Item = usize>) {
        let mut inner = lock_mutex(&self.inner);
        inner.free.extend(indices);
    }
}

struct AreaSegment {
    index: usize,
    base: NonNull<u8>,
    len: usize,
    cursor: usize,
}

impl AreaSegment {
    fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let aligned = align_up(self.cursor, layout.align())?;
        let end = aligned.checked_add(layout.size())?;
        if end > self.len {
            return None;
        }

        self.cursor = end;
        let ptr = unsafe { self.base.as_ptr().add(aligned) };
        NonNull::new(ptr)
    }
}

/// Fixed backing store used by [`AreaVec`], [`AreaBytes`], and [`AreaString`].
pub struct Area {
    pool: *const AreaPool,
    segments: Mutex<Vec<AreaSegment>>,
}

impl Area {
    fn pool(&self) -> &AreaPool {
        unsafe { &*self.pool }
    }

    /// Total checked-out capacity of this area in bytes.
    pub fn capacity(&self) -> usize {
        lock_mutex(&self.segments).iter().map(|s| s.len).sum()
    }

    /// Bytes already handed out by this area.
    pub fn bytes_used(&self) -> usize {
        lock_mutex(&self.segments).iter().map(|s| s.cursor).sum()
    }

    /// Bytes still available in the currently checked-out segments.
    pub fn bytes_remaining(&self) -> usize {
        let segments = lock_mutex(&self.segments);
        let capacity: usize = segments.iter().map(|s| s.len).sum();
        let used: usize = segments.iter().map(|s| s.cursor).sum();
        capacity.saturating_sub(used)
    }

    /// Resets this area and returns extra segments back to the pool.
    ///
    /// # Safety
    /// Caller must guarantee no live values still point into the area.
    pub unsafe fn reset(&self) {
        let mut segments = lock_mutex(&self.segments);
        if let Some((first, rest)) = segments.split_first_mut() {
            first.cursor = 0;
            let released: Vec<usize> = rest.iter().map(|s| s.index).collect();
            segments.truncate(1);
            self.pool().release_segments(released);
        }
    }

    fn allocate(&self, layout: Layout) -> Option<NonNull<u8>> {
        if layout.size() == 0 {
            return Some(NonNull::dangling());
        }

        let mut segments = lock_mutex(&self.segments);
        if let Some(segment) = segments.last_mut() {
            if let Some(ptr) = segment.allocate(layout) {
                return Some(ptr);
            }
        }

        let mut new_segment = self.pool().acquire_segment()?;
        let ptr = new_segment.allocate(layout)?;
        segments.push(new_segment);
        Some(ptr)
    }
}

impl Drop for Area {
    fn drop(&mut self) {
        let segments = lock_mutex(&self.segments);
        let released: Vec<usize> = segments.iter().map(|s| s.index).collect();
        drop(segments);
        self.pool().release_segments(released);
    }
}

fn align_up(offset: usize, align: usize) -> Option<usize> {
    let mask = align.checked_sub(1)?;
    offset.checked_add(mask).map(|v| v & !mask)
}

const INLINE_BYTE_CAP: usize = 23;

/// Variable-length sequence backed by an [`Area`].
pub struct AreaVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
    area: *const Area,
    _marker: PhantomData<T>,
}

impl<T> AreaVec<T> {
    /// Creates an empty vector. The first allocation will use the currently installed area.
    pub const fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            area: ptr::null(),
            _marker: PhantomData,
        }
    }

    /// Length in elements.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Capacity in elements.
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends an element to the vector using the current area if allocation is needed.
    pub fn push(&mut self, elem: T) -> Result<(), ()> {
        micropb::PbVec::pb_push(self, elem)
    }

    fn area_ptr(&self) -> Option<*const Area> {
        if self.area.is_null() {
            None
        } else {
            Some(self.area)
        }
    }

    fn area_or_current_ptr(&self) -> *const Area {
        self.area_ptr().unwrap_or_else(current_area_ptr)
    }

    fn reserve_internal(&mut self, additional: usize, area_ptr: *const Area) -> Result<(), ()> {
        let required = self.len.checked_add(additional).ok_or(())?;
        if required <= self.cap {
            if self.area.is_null() {
                self.area = area_ptr;
            }
            return Ok(());
        }

        let new_cap = required.max(self.cap.max(4)).next_power_of_two();
        let layout = Layout::array::<T>(new_cap).map_err(|_| ())?;
        let new_ptr = unsafe { &*area_ptr }.allocate(layout).ok_or(())?.cast::<T>();

        if self.len != 0 {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr.as_ptr(), new_ptr.as_ptr(), self.len);
            }
        }

        self.ptr = new_ptr;
        self.cap = new_cap;
        self.area = area_ptr;
        Ok(())
    }

    fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        let spare = self.cap - self.len;
        unsafe {
            core::slice::from_raw_parts_mut(
                self.ptr.as_ptr().add(self.len).cast::<MaybeUninit<T>>(),
                spare,
            )
        }
    }
}

impl<T> Default for AreaVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for AreaVec<T> {
    fn drop(&mut self) {
        if self.len != 0 {
            unsafe {
                ptr::drop_in_place(core::ptr::slice_from_raw_parts_mut(
                    self.ptr.as_ptr(),
                    self.len,
                ));
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for AreaVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for AreaVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T: Eq> Eq for AreaVec<T> {}

impl<T> Deref for AreaVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for AreaVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> micropb::PbVec<T> for AreaVec<T> {
    fn pb_push(&mut self, elem: T) -> Result<(), ()> {
        let area_ptr = self.area_or_current_ptr();
        self.reserve_internal(1, area_ptr)?;
        unsafe {
            self.ptr.as_ptr().add(self.len).write(elem);
        }
        self.len += 1;
        Ok(())
    }
}

enum AreaBytesStorage {
    Inline {
        len: u8,
        buf: [MaybeUninit<u8>; INLINE_BYTE_CAP],
    },
    Heap(AreaVec<u8>),
}

/// Arbitrary bytes stored in an [`Area`], with short values kept inline.
pub struct AreaBytes {
    storage: AreaBytesStorage,
}

impl AreaBytes {
    /// Creates an empty byte sequence.
    pub const fn new() -> Self {
        Self {
            storage: AreaBytesStorage::Inline {
                len: 0,
                buf: [MaybeUninit::uninit(); INLINE_BYTE_CAP],
            },
        }
    }

    /// Creates a byte container from a slice, spilling into the current area if needed.
    pub fn from_slice(data: &[u8]) -> Result<Self, ()> {
        let mut out = Self::new();
        micropb::PbString::pb_reserve(&mut out, data.len());
        let spare = micropb::PbString::pb_spare_cap(&mut out);
        if spare.len() < data.len() {
            return Err(());
        }
        spare[..data.len()].copy_from_slice(bytemuck_maybeuninit(data));
        unsafe { micropb::PbString::pb_set_len(&mut out, data.len()) };
        Ok(out)
    }

    fn len(&self) -> usize {
        match &self.storage {
            AreaBytesStorage::Inline { len, .. } => usize::from(*len),
            AreaBytesStorage::Heap(vec) => vec.len,
        }
    }

    fn spill_to_heap(&mut self, min_capacity: usize) -> Result<&mut AreaVec<u8>, ()> {
        if matches!(self.storage, AreaBytesStorage::Heap(_)) {
            let AreaBytesStorage::Heap(vec) = &mut self.storage else {
                unreachable!()
            };
            return Ok(vec);
        }

        let area_ptr = current_area_ptr();
        let mut vec = AreaVec::new();
        vec.reserve_internal(min_capacity, area_ptr)?;
        if let AreaBytesStorage::Inline { len, buf } = &self.storage {
            let len = usize::from(*len);
            for byte in &buf[..len] {
                micropb::PbVec::pb_push(&mut vec, unsafe { byte.assume_init() })?;
            }
        }
        self.storage = AreaBytesStorage::Heap(vec);
        let AreaBytesStorage::Heap(vec) = &mut self.storage else {
            unreachable!()
        };
        Ok(vec)
    }
}

impl Default for AreaBytes {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for AreaBytes {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for AreaBytes {}

impl fmt::Debug for AreaBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl Deref for AreaBytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match &self.storage {
            AreaBytesStorage::Inline { len, buf } => unsafe {
                core::slice::from_raw_parts(buf.as_ptr().cast::<u8>(), usize::from(*len))
            },
            AreaBytesStorage::Heap(vec) => vec.deref(),
        }
    }
}

impl DerefMut for AreaBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.storage {
            AreaBytesStorage::Inline { len, buf } => unsafe {
                core::slice::from_raw_parts_mut(buf.as_mut_ptr().cast::<u8>(), usize::from(*len))
            },
            AreaBytesStorage::Heap(vec) => vec.deref_mut(),
        }
    }
}

impl micropb::PbString for AreaBytes {
    unsafe fn pb_set_len(&mut self, len: usize) {
        match &mut self.storage {
            AreaBytesStorage::Inline { len: cur_len, .. } => {
                *cur_len = len.try_into().expect("inline byte length overflow");
            }
            AreaBytesStorage::Heap(vec) => vec.len = len,
        }
    }

    fn pb_reserve(&mut self, additional: usize) {
        let required = self.len().saturating_add(additional);
        match &mut self.storage {
            AreaBytesStorage::Inline { .. } if required <= INLINE_BYTE_CAP => {}
            AreaBytesStorage::Inline { .. } => {
                let _ = self.spill_to_heap(required);
            }
            AreaBytesStorage::Heap(vec) => {
                let area_ptr = vec.area_or_current_ptr();
                let _ = vec.reserve_internal(additional, area_ptr);
            }
        }
    }

    fn pb_clear(&mut self) {
        match &mut self.storage {
            AreaBytesStorage::Inline { len, .. } => *len = 0,
            AreaBytesStorage::Heap(vec) => vec.len = 0,
        }
    }

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
        match &mut self.storage {
            AreaBytesStorage::Inline { len, buf } => &mut buf[usize::from(*len)..],
            AreaBytesStorage::Heap(vec) => vec.spare_capacity_mut(),
        }
    }
}

impl micropb::PbBytes for AreaBytes {}

/// UTF-8 string stored in an [`Area`].
pub struct AreaString(pub AreaBytes);

impl AreaString {
    /// Creates an empty string.
    pub const fn new() -> Self {
        Self(AreaBytes::new())
    }

    /// Creates a string container from `text`, spilling into the current area if needed.
    pub fn from_str(text: &str) -> Result<Self, ()> {
        Ok(Self(AreaBytes::from_slice(text.as_bytes())?))
    }
}

impl Default for AreaString {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for AreaString {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for AreaString {}

impl fmt::Debug for AreaString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.deref().fmt(f)
    }
}

impl Deref for AreaString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(&self.0) }
    }
}

impl DerefMut for AreaString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.0) }
    }
}

impl micropb::PbString for AreaString {
    unsafe fn pb_set_len(&mut self, len: usize) {
        self.0.pb_set_len(len);
    }

    fn pb_reserve(&mut self, additional: usize) {
        self.0.pb_reserve(additional);
    }

    fn pb_clear(&mut self) {
        self.0.pb_clear();
    }

    fn pb_spare_cap(&mut self) -> &mut [MaybeUninit<u8>] {
        self.0.pb_spare_cap()
    }
}

fn bytemuck_maybeuninit(data: &[u8]) -> &[MaybeUninit<u8>] {
    unsafe { core::slice::from_raw_parts(data.as_ptr().cast::<MaybeUninit<u8>>(), data.len()) }
}
