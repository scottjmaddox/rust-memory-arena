// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw;
use std::mem::transmute;
use std::error::Error;
use std::fmt;
use std::result;

type Result<T> = result::Result<T, AllocError>;

//TODO: implement aligned_alloc for Windows, using _aligned_malloc
pub(crate) unsafe fn aligned_alloc(alignment: usize, size: usize) -> Result<*mut u8> {
    let mut mem: *mut raw::c_void = transmute(0_usize);
    if size == 0 {
        return Err(AllocError::ZeroSizeAlloc);
    }
    let errno = c::posix_memalign(&mut mem, alignment, size);
    if errno != 0 {
        Err(AllocError::Errno(errno))
    } else {
        Ok(transmute(mem))
    }
}

pub(crate) unsafe fn free(ptr: *mut u8) {
    c::free(transmute(ptr));
}

#[derive(Debug, PartialEq, Eq)]
pub enum AllocError {
    ZeroSizeAlloc,
    Errno(raw::c_int),
}

impl Error for AllocError {
    fn description(&self) -> &str {
        match *self {
            AllocError::ZeroSizeAlloc => "zero sized allocation is not supported",

            AllocError::Errno(_) => "system allocation error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AllocError::ZeroSizeAlloc => write!(f, "zero sized allocation is not supported"),

            AllocError::Errno(errno) => write!(f, "system allocation error number: {}", errno),
        }
    }
}

mod c {
    use std::os::raw;
    extern "C" {
        pub(crate) fn posix_memalign(
            mem: *mut *mut raw::c_void,
            alignment: usize,
            size: usize,
        ) -> raw::c_int;
        pub(crate) fn free(mem: *mut *mut raw::c_void);
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
            let ptr = aligned_alloc(alignment, ::std::mem::size_of::<isize>()).unwrap();
            let iptr: *mut isize = transmute(ptr);
            *iptr = 0;
            free(ptr);
        }
    }
}
