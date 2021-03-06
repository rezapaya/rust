// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions relating to memory layout

use unstable::intrinsics;

/// Returns the size of a type
#[inline]
pub fn size_of<T>() -> uint {
    unsafe { intrinsics::size_of::<T>() }
}

/// Returns the size of the type that `_val` points to
#[inline]
pub fn size_of_val<T>(_val: &T) -> uint {
    size_of::<T>()
}

/**
 * Returns the size of a type, or 1 if the actual size is zero.
 *
 * Useful for building structures containing variable-length arrays.
 */
#[inline]
pub fn nonzero_size_of<T>() -> uint {
    let s = size_of::<T>();
    if s == 0 { 1 } else { s }
}

/// Returns the size of the type of the value that `_val` points to
#[inline]
pub fn nonzero_size_of_val<T>(_val: &T) -> uint {
    nonzero_size_of::<T>()
}


/**
 * Returns the ABI-required minimum alignment of a type
 *
 * This is the alignment used for struct fields. It may be smaller
 * than the preferred alignment.
 */
#[inline]
pub fn min_align_of<T>() -> uint {
    unsafe { intrinsics::min_align_of::<T>() }
}

/// Returns the ABI-required minimum alignment of the type of the value that
/// `_val` points to
#[inline]
pub fn min_align_of_val<T>(_val: &T) -> uint {
    min_align_of::<T>()
}

/// Returns the preferred alignment of a type
#[inline]
pub fn pref_align_of<T>() -> uint {
    unsafe { intrinsics::pref_align_of::<T>() }
}

/// Returns the preferred alignment of the type of the value that
/// `_val` points to
#[inline]
pub fn pref_align_of_val<T>(_val: &T) -> uint {
    pref_align_of::<T>()
}

#[cfg(test)]
mod tests {
    use cast;
    use mem::*;

    #[test]
    fn size_of_basic() {
        assert_eq!(size_of::<u8>(), 1u);
        assert_eq!(size_of::<u16>(), 2u);
        assert_eq!(size_of::<u32>(), 4u);
        assert_eq!(size_of::<u64>(), 8u);
    }

    #[test]
    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "arm")]
    #[cfg(target_arch = "mips")]
    fn size_of_32() {
        assert_eq!(size_of::<uint>(), 4u);
        assert_eq!(size_of::<*uint>(), 4u);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn size_of_64() {
        assert_eq!(size_of::<uint>(), 8u);
        assert_eq!(size_of::<*uint>(), 8u);
    }

    #[test]
    fn size_of_val_basic() {
        assert_eq!(size_of_val(&1u8), 1);
        assert_eq!(size_of_val(&1u16), 2);
        assert_eq!(size_of_val(&1u32), 4);
        assert_eq!(size_of_val(&1u64), 8);
    }

    #[test]
    fn nonzero_size_of_basic() {
        type Z = [i8, ..0];
        assert_eq!(size_of::<Z>(), 0u);
        assert_eq!(nonzero_size_of::<Z>(), 1u);
        assert_eq!(nonzero_size_of::<uint>(), size_of::<uint>());
    }

    #[test]
    fn nonzero_size_of_val_basic() {
        let z = [0u8, ..0];
        assert_eq!(size_of_val(&z), 0u);
        assert_eq!(nonzero_size_of_val(&z), 1u);
        assert_eq!(nonzero_size_of_val(&1u), size_of_val(&1u));
    }

    #[test]
    fn align_of_basic() {
        assert_eq!(pref_align_of::<u8>(), 1u);
        assert_eq!(pref_align_of::<u16>(), 2u);
        assert_eq!(pref_align_of::<u32>(), 4u);
    }

    #[test]
    #[cfg(target_arch = "x86")]
    #[cfg(target_arch = "arm")]
    #[cfg(target_arch = "mips")]
    fn align_of_32() {
        assert_eq!(pref_align_of::<uint>(), 4u);
        assert_eq!(pref_align_of::<*uint>(), 4u);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn align_of_64() {
        assert_eq!(pref_align_of::<uint>(), 8u);
        assert_eq!(pref_align_of::<*uint>(), 8u);
    }

    #[test]
    fn align_of_val_basic() {
        assert_eq!(pref_align_of_val(&1u8), 1u);
        assert_eq!(pref_align_of_val(&1u16), 2u);
        assert_eq!(pref_align_of_val(&1u32), 4u);
    }
}
