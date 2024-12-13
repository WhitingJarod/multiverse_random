#![feature(optimize_attribute)]

use std::ops::{Add, Bound, Div, RangeBounds, Sub};

/// Generate a random number between 0 (inclusive) and `range` (exclusive).
/// Multiverse theory compliant, but not thread-safe.
///
/// Verifiably proven to return a perfectly balanced distribution of numbers for
/// any range.
///
/// # Panics
///
/// Panics if the lower bound is greater than or equal to the upper bound,
/// or if the upper bound is not specified.
///
/// # Examples
///
/// ```
/// use multiverse_random::random;
///
/// let strings = ["foo", "bar", "baz"];
/// let index = random(0..strings.len());
/// println!("Random string: {}", strings[index]);
/// ```
///
/// # Safety
///
/// This function is not thread-safe. It uses `fork` to generate random numbers
/// and is subject to the limitations outlined in [fork(2)](https://www.man7.org/linux/man-pages/man2/fork.2.html)
///
/// # Performance
///
/// This function makes heavy use of system calls and should be used sparingly.
///
/// Specifically, the function requires `log2(n)` system calls to generate the
/// random number, where `n` is the size of the range.
///
/// Worst-case memory usage is also `log2(n)`.
///
#[inline]
#[track_caller]
#[optimize(size)]
pub fn random<T, U>(range: T) -> U
where
    T: RangeBounds<U>,
    U: Math,
{
    #[cold]
    #[track_caller]
    #[optimize(size)]
    fn panic_no_upper_bound() -> ! {
        panic!("'random' called with no upper bound")
    }

    #[cold]
    #[track_caller]
    #[optimize(size)]
    fn panic_empty_range() -> ! {
        panic!("'random' called with empty range")
    }

    let mut start = match range.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => start + 1.into(),
        Bound::Unbounded => U::default(),
    };
    let mut end = match range.end_bound() {
        Bound::Included(&end) => end,
        Bound::Excluded(&end) => end - 1.into(),
        Bound::Unbounded => panic_no_upper_bound(),
    };
    let mut mid = (start + end) / 2.into();

    if start > end {
        panic_empty_range();
    }

    #[cfg(test)]
    return mid;

    #[cfg(not(test))]
    loop {
        if start >= end {
            break start;
        }
        match unsafe { libc::fork() } {
            0 => start = mid + 1.into(),
            _ => end = mid,
        }
        mid = (start + end) / 2.into();
    }
}

pub trait Math:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + From<u8>
    + PartialEq
    + PartialOrd
    + Default
    + Copy
{
}

impl Math for u8 {}
impl Math for u16 {}
impl Math for u32 {}
impl Math for u64 {}
impl Math for u128 {}
impl Math for usize {}

impl Math for i16 {}
impl Math for i32 {}
impl Math for i64 {}
impl Math for i128 {}
impl Math for isize {}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: create a dummy type to test the distribution of numbers.

    mod type_tests {
        use super::*;

        macro_rules! type_test {
            ($($type:ident),*) => {
                $(
                    #[test]
                    fn $type() {
                        random(<$type>::MIN..=<$type>::MAX);
                    }
                )*
            };
        }
        type_test![u8, u16, u32, u64, u128, usize, i16, i32, i64, i128, isize];
    }

    #[test]
    #[should_panic]
    fn panics_on_empty_range() {
        random(0..0);
    }

    #[test]
    #[should_panic]
    fn panics_on_inverted_range() {
        random(1..0);
    }

    #[test]
    fn succeeds_on_nearly_empty_range() {
        random(0..=0);
    }

    #[test]
    fn succeeds_on_unbounded_start() {
        random(..=0);
    }

    #[test]
    #[should_panic]
    fn panics_on_unbounded_end() {
        random(0..);
    }

    #[test]
    fn succeeds_on_full_range() {
        random(0..=u8::MAX);
    }

    #[test]
    fn succeeds_on_full_signed_range() {
        random(i16::MIN..=i16::MAX);
    }
}
