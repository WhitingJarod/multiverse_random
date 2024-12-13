#![feature(optimize_attribute)]

/// Select a random number or item from the provided range.
/// Multiverse theory compliant.
///
/// Verifiably proven to return a perfectly balanced distribution of selections
/// for any number of items.
///
/// # Panics
///
/// Panics if the provided iterator has more than `isize::MAX` items.
///
/// For testing purposes, the function may also panic if size hints are not available.
/// To enable these panics, disable the `no_size_hints` feature.
///
/// # Examples
///
/// Basic usage is straightforward:
/// ```
/// use multiverse_random::random;
///
/// let strings = ["foo", "bar", "baz"];
/// let choice = random(strings);
/// println!("Random string: {}", choice);
/// ```
/// The function accepts standard range types:
/// ```
/// use multiverse_random::random;
///
/// let _ = random(0..=0);
/// let _ = random(5..15);
/// let _ = random(-10..=10);
/// ```
///
/// As well as anything that implements `IntoIterator`:
/// ```
/// use multiverse_random::random;
///
/// let vec = vec![5, 10, 15, 20];
/// let _ = random(vec);
///
/// let letters = "The quick brown fox jumps over the lazy dog";
/// let _ = random(letters.as_bytes());
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
/// random item, where `n` is the number of items.
///
/// Worst-case memory usage is also `O(log2 n)`.
///
#[inline]
#[track_caller]
#[optimize(size)]
#[must_use = "if you don't need the result, use a different system for multithreading"]
pub fn random<T, U>(items: U) -> T
where
    U: IntoIterator<Item = T>,
{
    #[cold]
    #[track_caller]
    #[inline(never)]
    #[optimize(size)]
    fn panic_out_of_range() -> ! {
        panic!(
            "'random' has too many values to choose from.\nthe maximum allowed is {}",
            isize::MAX
        );
    }

    #[cold]
    #[track_caller]
    #[inline(never)]
    #[optimize(size)]
    #[cfg(any(not(feature = "no_size_hints"), test))]
    fn panic_no_size_hints() -> ! {
        #[cfg(test)]
        panic!("the iterator sent to 'random' is not providing size hints. enable feature 'no_size_hints' to disable this check.");
        #[cfg(not(test))]
        std::process::exit(0);
    }

    #[cold]
    #[track_caller]
    #[inline(never)]
    #[optimize(size)]
    fn exit_no_choices() -> ! {
        #[cfg(test)]
        panic!("'random' has no values to choose from");
        #[cfg(not(test))]
        std::process::exit(0);
    }

    let range = items.into_iter();

    if let (_, Some(len)) = range.size_hint() {
        if len > isize::MAX as usize {
            panic_out_of_range();
        }
        #[cfg(test)]
        if len > i32::MAX as usize {
            let mut range = range;
            return range.next().unwrap();
        }
    } else {
        #[cfg(any(not(feature = "no_size_hints"), test))]
        panic_no_size_hints();
    }

    let set = range.collect::<Vec<T>>();
    let end = set.len();

    if end == 0 {
        exit_no_choices();
    }

    #[cfg_attr(test, allow(unused_mut))]
    let mut start = 0;
    #[cfg_attr(test, allow(unused_mut))]
    let mut end = end as isize;

    let set = set.as_ptr();

    #[cfg_attr(test, allow(unused_mut))]
    let mut mid = (start + end) / 2;

    #[cfg(test)]
    return unsafe { std::ptr::read(set.offset(mid)) };

    #[cfg_attr(test, allow(unreachable_code))]
    loop {
        if start == end {
            break unsafe { std::ptr::read(set.offset(start)) };
        }
        match unsafe { libc::fork() } {
            0 => start = mid + 1,
            _ => end = mid,
        }
        mid = (start + end) / 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO: create a dummy type to test the distribution of numbers.

    mod type_tests {
        use super::*;

        macro_rules! type_test {
            (succeed: [$($type:ident),*], fail: [$($type2:ident),*]) => {
                $(
                    #[test]
                    fn $type() {
                        let _ = random(<$type>::MIN..=<$type>::MAX);
                    }
                )*

                $(
                    #[test]
                    #[should_panic]
                    fn $type2() {
                        let _ = random(<$type2>::MIN..=<$type2>::MAX);
                    }
                )*
            };
        }

        type_test! {
            succeed: [u8, u16, u32, i8, i16, i32],
            fail: [u64, usize, i64, isize]
        }
    }

    #[test]
    #[should_panic]
    fn panics_on_empty_range() {
        let _ = random(0..0);
    }

    #[test]
    #[should_panic]
    fn panics_on_inverted_range() {
        let _ = random(1..0);
    }

    #[test]
    fn succeeds_on_nearly_empty_range() {
        let _ = random(0..=0);
    }

    #[test]
    #[should_panic]
    fn panics_on_unbounded_end() {
        let _ = random(0..);
    }

    #[test]
    fn succeeds_on_strings() {
        let strings = ["foo", "bar", "baz"];
        let _ = random(strings);
    }
}
