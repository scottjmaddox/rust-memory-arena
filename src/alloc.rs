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
use libc::{c_int, c_void};


#[cfg(not(windows))]
pub(crate) use libc::posix_memalign;
// #[cfg(windows)]
// extern "C" fn _aligned_malloc(size: usize, alignment: usize) -> *mut u8;
pub(crate) use libc::free as c_free;

type Result<T> = result::Result<T, AllocError>;

//TODO: implement aligned_alloc for Windows, using _aligned_malloc
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
