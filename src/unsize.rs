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

/// Types that can be "unsized" to a dynamically-sized type.
///
/// For example, the sized array type `[i8; 2]` implements `Unsize<[i8]>` and
/// `Unsize<fmt::Debug>`.
///
/// All implementations of `Unsize` are provided automatically by the compiler.
///
/// `Unsize` is implemented for:
///
/// - `[T; N]` is `Unsize<[T]>`
/// - `T` is `Unsize<Trait>` when `T: Trait`
/// - `Foo<..., T, ...>` is `Unsize<Foo<..., U, ...>>` if:
///   - `T: Unsize<U>`
///   - Foo is a struct
///   - Only the last field of `Foo` has a type involving `T`
///   - `T` is not part of the type of any other fields
///   - `Bar<T>: Unsize<Bar<U>>`, if the last field of `Foo` has type `Bar<T>`
///
/// `Unsize` is used along with [`ops::CoerceUnsized`][coerceunsized] to allow
/// "user-defined" containers such as [`rc::Rc`][rc] to contain dynamically-sized
/// types. See the [DST coercion RFC][RFC982] and [the nomicon entry on coercion][nomicon-coerce]
/// for more details.
///
/// [coerceunsized]: ../ops/trait.CoerceUnsized.html
/// [rc]: ../../std/rc/struct.Rc.html
/// [RFC982]: https://github.com/rust-lang/rfcs/blob/master/text/0982-dst-coercion.md
/// [nomicon-coerce]: ../../nomicon/coercions.html
pub trait Unsize<T: ?Sized> {
    // Empty.
}
