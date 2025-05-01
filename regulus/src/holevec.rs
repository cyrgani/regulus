use std::alloc::{self, Layout};
use std::mem::forget;
use std::ops::Deref;
use std::ptr::{self, NonNull};

/// An alternative to [`Vec<T>`] which allows moving elements out of it.
/// unsafe to use!!!
pub struct HoleVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> HoleVec<T> {
    pub const fn new() -> Self {
        assert!(size_of::<T>() != 0, "We're not ready to handle ZSTs");

        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
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

    const unsafe fn read(&self, index: usize) -> T {
        unsafe { ptr::read(self.ptr.as_ptr().add(index)) }
    }

    /// unsafe to call twice on the same idx!!!
    pub const fn at(&self, index: usize) -> T {
        assert!(index < self.len);
        unsafe { self.read(index) }
    }

    /// unsafe to call twice on the same idx!!!
    pub const fn get(&self, index: usize) -> Option<T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.read(index) })
        }
    }

    const fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn split_first(mut self) -> Option<(T, Self)> {
        if self.len == 0 {
            None
        } else {
            let el = self.at(0);
            self.ptr = unsafe { self.ptr.add(1) };
            self.len -= 1;
            self.cap -= 1;
            Some((el, self))
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
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr().cast::<u8>(), layout);
            }
        }
    }
}

impl<T> From<HoleVec<T>> for Vec<T> {
    fn from(value: HoleVec<T>) -> Self {
        let v = unsafe { Self::from_raw_parts(value.ptr.as_ptr(), value.len, value.cap) };
        forget(value);
        v
    }
}

impl<T> IntoIterator for HoleVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}
