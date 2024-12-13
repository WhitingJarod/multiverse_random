#![feature(optimize_attribute)]

use std::ops::{Add, Bound, Div, RangeBounds, Sub};

/// Generate a random number within the provided range.
/// Multiverse theory compliant.
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
/// Basic usage is straightforward:
/// ```
/// use multiverse_random::random;
///
/// let strings = ["foo", "bar", "baz"];
/// let index = random(..strings.len());
/// println!("Random string: {}", strings[index]);
/// ```
/// The function accepts standard range types:
/// ```
/// use multiverse_random::random;
///
/// let _ = random(-2..=-1);
/// let _ = random(..10);
/// let _ = random(0..=0);
/// ```
///
/// But it will fail on empty or inverted ranges or if the upper bound is not
/// specified:
/// ```should_panic
/// use multiverse_random::random;
///
/// let _ = random(0..0);
/// // -> 'random' called with empty range
/// let _ = random(1..0);
/// // -> 'random' called with empty range
/// let _ = random(0..);
/// // -> 'random' called with no upper bound
/// ```
///
/// # Safety
///
/// **Guaranteed safe until printed date.**
/// Warranty void if used for production code.
///
/// # Performance
///
/// This function makes heavy use of system calls and should be used sparingly.
///
/// Specifically, the function requires `log2 n` system calls to generate the
/// random number, where `n` is the size of the range.
///
/// Worst-case memory usage is also `O(log2 n)`.
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
    #[inline(never)]
    #[optimize(size)]
    fn panic_no_upper_bound() -> ! {
        panic!("'random' called with no upper bound")
    }

    #[cold]
    #[track_caller]
    #[inline(never)]
    #[optimize(size)]
    fn panic_empty_range() -> ! {
        panic!("'random' called with empty range")
    }

    // Prefer U's implementation of Copy rather than U's implementation
    // of From<u8>. There's no way to tell at this stage which has less
    // overhead, but this is the more likely case.
    // In any event, optimizations at this stage are of little consequence
    // due to the sheer volume of system calls being invoked later.
    let n = (0.into(), 1.into(), 2.into());

    #[cfg_attr(test, allow(unused_mut))]
    let mut start = match range.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => start + n.1,
        Bound::Unbounded => U::default(),
    };
    #[cfg_attr(test, allow(unused_mut))]
    let mut end = match range.end_bound() {
        Bound::Included(&end) => end,
        Bound::Excluded(&end) => end - n.1,
        Bound::Unbounded => panic_no_upper_bound(),
    };
    #[cfg_attr(test, allow(unused_mut))]
    let mut mid = (start + end) / n.2;

    if start > end {
        panic_empty_range();
    }

    // Short circuit and return during tests.
    // Forking during a test is pointless, so return something until
    // such time as I've hacked up some kind of testing support.
    // (the cfg_attrs above should be removed once this is finished)
    // See the TODO in the tests module.
    #[cfg(test)]
    return mid;

    #[cfg_attr(test, allow(unreachable_code))]
    loop {
        if start >= end {
            break start;
        }
        match unsafe { libc::fork() } {
            0 => start = mid + n.1,
            _ => end = mid,
        }
        mid = (start + end) / n.2;
        if mid < n.0 {
            // Note to self: dividing negative integers always rounds twoards zero, not down.
            // Delete this line if you want to forkbomb yourself.
            mid = mid - n.1;
        }
    }
}

/// A trait for types that can be used with `random`.
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
}
