// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::mem::transmute;
use core::fmt;
use core::result;
#[allow(unused_imports)]
use libc::{c_int, c_void, size_t};


#[cfg(not(windows))]
pub(crate) use libc::posix_memalign;
#[cfg(windows)]
extern {
    fn _aligned_malloc(size: size_t, alignment: size_t) -> *mut c_void;
    fn _get_errno(p: *mut c_int) -> c_int;
}
pub(crate) use libc::free as c_free;

type Result<T> = result::Result<T, AllocError>;

#[cfg(not(windows))]
pub(crate) unsafe fn aligned_alloc(size: usize, alignment: usize) -> Result<*mut u8> {
    let mut mem: *mut c_void = transmute(0_usize);
    if size == 0 {
        return Err(AllocError::ZeroSizeAlloc);
    }
    let errno = posix_memalign(&mut mem, alignment, size);
    if errno != 0 {
        Err(AllocError::Errno(errno))
    } else {
        Ok(transmute(mem))
    }
}

#[cfg(windows)]
pub(crate) unsafe fn aligned_alloc(size: usize, alignment: usize) -> Result<*mut u8> {
    if size == 0 {
        return Err(AllocError::ZeroSizeAlloc);
    }
    let mut mem: *mut c_void = _aligned_malloc(size, alignment);
    if mem.is_null() {
        let errno: c_int = 0;
        _get_errno(&mut errno);
        Err(AllocError::Errno(errno))
    } else {
        Ok(transmute(mem))
    }
}

pub(crate) unsafe fn free(ptr: *mut u8) {
    c_free(transmute(ptr));
}

#[derive(Debug, PartialEq, Eq)]
pub enum AllocError {
    ZeroSizeAlloc,
    Errno(c_int),
}

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocError::ZeroSizeAlloc => write!(f, "zero sized allocation is not supported"),

            AllocError::Errno(errno) => write!(f, "system allocation error number: {}", errno),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn aligned_alloc_and_free() {
        unsafe {
            let alignment = 1024;
            let ptr = aligned_alloc(alignment, ::core::mem::size_of::<isize>()).unwrap();
            let iptr: *mut isize = transmute(ptr);
            *iptr = 0;
            free(ptr);
        }
    }
}
