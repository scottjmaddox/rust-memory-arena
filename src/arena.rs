// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cell::Cell;
use arena_box::ArenaBox;

pub struct Arena {
    size: usize,
    used: Cell<usize>,
    mem: *mut u8,
}

impl Arena {
    pub fn new(alignment: usize, size: usize) -> Result<Self, ::alloc::AllocError> {
        if size == 0 {
            Ok(Self {
                size: size,
                used: Cell::new(0),
                mem: 1 as *mut u8,
            })
        } else {
            unsafe {
                let mem = ::alloc::aligned_alloc(alignment, size)?;
                Ok(Self {
                    size: size,
                    used: Cell::new(0),
                    mem: mem,
                })
            }
        }
    }

    #[inline]
    fn padding<T>() -> usize {
        let size = ::core::mem::size_of::<T>();
        let alignment = ::core::mem::align_of::<T>();
        let rem = size % alignment;
        if rem == 0 {
            0
        } else {
            alignment - rem
        }
    }

    fn alloc<T>(&self) -> Option<*mut T> {
        let size = ::core::mem::size_of::<T>();
        if size == 0 {
            return Some(::core::mem::align_of::<T>() as *mut T);
        }
        let padding = Self::padding::<T>();
        if self.used.get() + size + padding > self.size {
            return None;
        }
        let p = (self.mem as usize + self.used.get()) as *mut T;
        self.used.set(self.used.get() + size + padding);
        Some(p)
    }

    /// Allocates memory from the Arena, places x into it,
    /// and returns the resulting `ArenaBox`, wrapped in `Result::Ok`.
    ///
    /// If there is not enough available memory in the Arena,
    /// then the original value is returned, wrapped in `Result::Err`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use memory_arena::*;
    /// let alignment = 1024;
    /// let size = 1024;
    /// let a = Arena::new(alignment, size).unwrap();
    /// let mut num = a.new_box(42).unwrap();
    /// assert_eq!(*num, 42);
    /// *num += 1;
    /// assert_eq!(*num, 43);
    /// ```
    ///
    /// The following example shows the behavior when the
    /// Arena does not have enough remaining memory
    /// to fit `x`.
    ///
    /// ```
    /// # use memory_arena::*;
    /// let alignment = 512;
    /// let size = 1;
    /// let a = Arena::new(alignment, size).unwrap();
    /// let i: usize = 42;
    /// assert_eq!(a.new_box(i), Err(42));
    /// ```
    ///
    /// The following example will not compile, because the ArenaBox
    /// cannot outlive the Arena it was allocated from.
    ///
    /// ```compile_fail
    /// # use memory_arena::*;
    /// let outer_num = {
    ///     let a = Arena::new(512, 512).unwrap();
    ///     //  ^ borrowed value does not live long enough
    ///     let num: ArenaBox<usize> = a.new_box(0).unwrap();
    ///     num
    /// }; // `a` dropped here while still borrowed
    /// // borrowed value needs to live until here
    /// ```
    pub fn new_box<'a, T>(&'a self, x: T) -> Result<ArenaBox<'a, T>, T> {
        match self.alloc::<T>() {
            None => Err(x),
            Some(p) => {
                unsafe {
                    *p = x;
                }
                Ok(unsafe { ArenaBox::from_raw(p) })
            }
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        unsafe {
            ::alloc::free(self.mem);
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn memory_arena_box() {
        let alignment = 1024;
        let size = 1024;
        let a = Arena::new(alignment, size).unwrap();
        let mut num = a.new_box(42).unwrap();
        assert_eq!(*num, 42);
        *num += 1;
        assert_eq!(*num, 43);
    }
    #[test]
    fn memory_arena_out_of_memory() {
        let alignment = 512;
        let size = 1;
        let a = Arena::new(alignment, size).unwrap();
        let i: usize = 42;
        assert_eq!(a.new_box(i), Err(42));
    }
}
