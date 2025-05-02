use std::alloc::{self, Layout};
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

/// An alternative to [`Vec<T>`] which allows moving elements out of it.
/// unsafe to use!!!
pub struct HoleVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    // bool i describes if element i was already dropped (then true)
    dropped_flags: Vec<AtomicBool>,
}

impl<T> Default for HoleVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HoleVec<T> {
    pub const fn new() -> Self {
        assert!(size_of::<T>() != 0, "HoleVec does not need to support ZSTs");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
            dropped_flags: vec![],
        }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        let mut v = Self::new();
        for el in data {
            v.push(el);
        }
        v
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }

        // Can't fail, we'll OOM first.
        self.len += 1;
        self.dropped_flags.push(AtomicBool::new(false));
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            isize::try_from(new_layout.size()).is_ok(),
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr().cast::<u8>();
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr.cast::<T>()) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    unsafe fn read(&self, index: usize) -> T {
        debug_assert!(!self.dropped_flags[index].load(Ordering::SeqCst));
        
        self.dropped_flags.get(index).unwrap().store(true, Ordering::SeqCst);
        unsafe { ptr::read(self.ptr.as_ptr().add(index)) }
    }

    /// UB to call twice on the same idx!!!
    pub fn at(&self, index: usize) -> T {
        assert!(index < self.len);
        unsafe { self.read(index) }
    }

    /// UB to call twice on the same idx!!!
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.read(index) })
        }
    }
}

impl<T> Deref for HoleVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for HoleVec<T> {
    fn drop(&mut self) {
        for i in 0..self.len {
            if !self.dropped_flags[i].load(Ordering::SeqCst) {
                self.at(i);
            }
        }
        
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr().cast::<u8>(), layout);
            }
        }
    }
}

impl<T> IntoIterator for HoleVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { vec: self, idx: 0 }
    }
}

pub struct IntoIter<T> {
    vec: HoleVec<T>,
    idx: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let el = self.vec.get(self.idx)?;
        self.idx += 1;
        Some(el)
    }
}
