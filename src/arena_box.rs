// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This file has been modified from the original version in the
// Rust core and/or standard library. The original copyright is below:
//
// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A pointer type for a value that lives in a `Arena`.
//!
//! `ArenaBox<T>`, casually referred to as an 'arena box', provides a safe
//! interface around allocation from a memory arena allocation. Arena boxes
//! provide ownership for the allocated value, and drops their contained value
//! when they go out of scope.
//!
//! # Examples
//!
//! Creating an arena box:
//!
//! ```
//! # use memory_arena::*;
//! let a = Arena::new(1024, 1024).unwrap();
//! let x = a.new_box(5).unwrap();
//! ```

use core::borrow;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{self, Hash, Hasher};
use core::ops::{Deref, DerefMut};
use core::marker::PhantomData;

use unique::Unique;
use Arena;

/// A pointer type for a value that lives in a `Arena`.
///
/// See the [module-level documentation](../arena_box/) for more.
pub struct ArenaBox<'a, T: ?Sized> {
    value: Unique<T>,
    phantom: PhantomData<&'a Arena>,
}

impl<'a, T: ?Sized> ArenaBox<'a, T> {
    /// Constructs an arena box from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `ArenaBox`. Specifically, the `ArenaBox` destructor will call
    /// the destructor of `T`. Since the
    /// way `ArenaBox` allocates and releases memory is unspecified, the
    /// only valid pointer to pass to this function is the one taken
    /// from another `ArenaBox` via the [`ArenaBox::into_raw`] function.
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    ///
    /// [`ArenaBox::into_raw`]: struct.ArenaBox.html#method.into_raw
    ///
    /// # Examples
    ///
    /// ```
    /// # use memory_arena::*;
    /// let a = Arena::new(1024, 1024).unwrap();
    /// let x = a.new_box(5).unwrap();
    /// let ptr = ArenaBox::into_raw(x);
    /// let x = unsafe { ArenaBox::from_raw(ptr) };
    /// ```
    #[inline]
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        ArenaBox {
            value: Unique::new_unchecked(raw),
            phantom: PhantomData,
        }
    }

    /// Consumes the `ArenaBox`, returning the wrapped raw pointer.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `ArenaBox`. In particular, the
    /// caller should properly destroy `T`, by calling
    /// `std::ptr::drop_in_place` on the pointer.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `ArenaBox::into_raw(b)` instead of `b.into_raw()`. This
    /// is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use memory_arena::*;
    /// let a = Arena::new(1024, 1024).unwrap();
    /// let x = a.new_box(5).unwrap();
    /// let ptr = ArenaBox::into_raw(x);
    /// ```
    pub fn into_raw(b: ArenaBox<T>) -> *mut T {
        let p = b.value.as_ptr();
        ::core::mem::forget(b);
        p
    }
}

impl<'a, T: ?Sized> Drop for ArenaBox<'a, T> {
    fn drop(&mut self) {
        unsafe { ::core::ptr::drop_in_place(self.value.as_ptr()) }
    }
}

impl<'a, T: ?Sized + PartialEq> PartialEq for ArenaBox<'a, T> {
    #[inline]
    fn eq(&self, other: &ArenaBox<T>) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    #[inline]
    fn ne(&self, other: &ArenaBox<T>) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}

impl<'a, T: ?Sized + PartialOrd> PartialOrd for ArenaBox<'a, T> {
    #[inline]
    fn partial_cmp(&self, other: &ArenaBox<T>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    #[inline]
    fn lt(&self, other: &ArenaBox<T>) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    #[inline]
    fn le(&self, other: &ArenaBox<T>) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    #[inline]
    fn ge(&self, other: &ArenaBox<T>) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    #[inline]
    fn gt(&self, other: &ArenaBox<T>) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}

impl<'a, T: ?Sized + Ord> Ord for ArenaBox<'a, T> {
    #[inline]
    fn cmp(&self, other: &ArenaBox<T>) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl<'a, T: ?Sized + Eq> Eq for ArenaBox<'a, T> {}

impl<'a, T: ?Sized + Hash> Hash for ArenaBox<'a, T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<'a, T: ?Sized + Hasher> Hasher for ArenaBox<'a, T> {
    fn finish(&self) -> u64 {
        (**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (**self).write(bytes)
    }
    fn write_u8(&mut self, i: u8) {
        (**self).write_u8(i)
    }
    fn write_u16(&mut self, i: u16) {
        (**self).write_u16(i)
    }
    fn write_u32(&mut self, i: u32) {
        (**self).write_u32(i)
    }
    fn write_u64(&mut self, i: u64) {
        (**self).write_u64(i)
    }
    fn write_usize(&mut self, i: usize) {
        (**self).write_usize(i)
    }
    fn write_i8(&mut self, i: i8) {
        (**self).write_i8(i)
    }
    fn write_i16(&mut self, i: i16) {
        (**self).write_i16(i)
    }
    fn write_i32(&mut self, i: i32) {
        (**self).write_i32(i)
    }
    fn write_i64(&mut self, i: i64) {
        (**self).write_i64(i)
    }
    fn write_isize(&mut self, i: isize) {
        (**self).write_isize(i)
    }
}

//TODO: fix this
// impl<'a> ArenaBox<'a, Any> {
//     #[inline]
//     /// Attempt to downcast the box to a concrete type.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use memory_arena::*;
//     /// use core::any::Any;
//     ///
//     /// fn print_if_string(value: ArenaBox<Any>) {
//     ///     if let Ok(string) = value.downcast::<String>() {
//     ///         println!("String ({}): {}", string.len(), string);
//     ///     }
//     /// }
//     ///
//     /// fn main() {
//     ///     let a = Arena::new(1024, 1024).unwrap();
//     ///     let my_string = "Hello World".to_string();
//     ///     print_if_string(a.new_box(my_string).unwrap());
//     ///     print_if_string(a.new_box(0i8).unwrap());
//     /// }
//     /// ```
//     pub fn downcast<T: Any>(self) -> Result<ArenaBox<'a, T>, ArenaBox<'a, Any>> {
//         if self.is::<T>() {
//             unsafe {
//                 let raw: *mut Any = ArenaBox::into_raw(self);
//                 Ok(ArenaBox::from_raw(raw as *mut T))
//             }
//         } else {
//             Err(self)
//         }
//     }
// }

//TODO: fix this
// impl<'a> ArenaBox<'a, Any + Send> {
//     #[inline]
//     /// Attempt to downcast the box to a concrete type.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// # use memory_arena::*;
//     /// use core::any::Any;
//     ///
//     /// fn print_if_string<'a>(value: ArenaBox<'a, Any + Send>) {
//     ///     if let Ok(string) = value.downcast::<String>() {
//     ///         println!("String ({}): {}", string.len(), string);
//     ///     }
//     /// }
//     ///
//     /// fn main() {
//     ///     let my_string = "Hello World".to_string();
//     ///     let a = Arena::new(1024, 1024).unwrap();
//     ///     print_if_string(a.new_box(my_string).unwrap());
//     ///     print_if_string(a.new_box(0i8).unwrap());
//     /// }
//     /// ```
//     pub fn downcast<T: Any>(self) -> Result<ArenaBox<'a, T>, ArenaBox<'a, Any + Send>> {
//         let s: ArenaBox<'a, Any + 'static> = unsafe { transmute(self) };
//         <ArenaBox<'a, Any>>::downcast(s).map_err(|s| unsafe {
//             // reapply the Send marker
//             ArenaBox::from_raw(ArenaBox::into_raw(s) as *mut (Any + Send))
//         })
//     }
// }

impl<'a, T: fmt::Display + ?Sized> fmt::Display for ArenaBox<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T: fmt::Debug + ?Sized> fmt::Debug for ArenaBox<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized> fmt::Pointer for ArenaBox<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // It's not possible to extract the inner Uniq directly from the ArenaBox,
        // instead we cast it to a *const which aliases the Unique
        let ptr: *const T = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<'a, T: ?Sized> Deref for ArenaBox<'a, T> {
    type Target = T;

    #[warn(unconditional_recursion)]
    fn deref(&self) -> &T {
        unsafe { self.value.as_ref() }
    }
}

impl<'a, T: ?Sized> DerefMut for ArenaBox<'a, T> {
    #[warn(unconditional_recursion)]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.value.as_mut() }
    }
}

impl<'a, I: Iterator + ?Sized> Iterator for ArenaBox<'a, I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
}

impl<'a, I: DoubleEndedIterator + ?Sized> DoubleEndedIterator for ArenaBox<'a, I> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
}

impl<'a, T: ?Sized> borrow::Borrow<T> for ArenaBox<'a, T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<'a, T: ?Sized> borrow::BorrowMut<T> for ArenaBox<'a, T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<'a, T: ?Sized> AsRef<T> for ArenaBox<'a, T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<'a, T: ?Sized> AsMut<T> for ArenaBox<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn ln112() {
        let a = Arena::new(1024, 1024).unwrap();
        let x = a.new_box(5).unwrap();
        let ptr = ArenaBox::into_raw(x);
        let _ = unsafe { ArenaBox::from_raw(ptr) };
    }

    #[test]
    fn ln140() {
        let a = Arena::new(1024, 1024).unwrap();
        let x = a.new_box(5).unwrap();
        let _ = ArenaBox::into_raw(x);
    }
}