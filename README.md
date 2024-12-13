# Multiverse Random

**Multiverse Random** provides a single function
```rust
random<T, U>(items: U) -> T
where U: IntoIterator<Item = T>
```
The function takes some list or iterator over items of type T, selects one, and returns it.

The catch?
For a list of `n` items, the function returns `n` times.

It does this by using the `fork` syscall, which will be called `log2(n)` times.

# License
The code is protected under the `Why Would You Ever Even Use This` standard license agreement.
This means that you are free to do literally anything with the code. Literally anything.

It's free, it's open source, it's garbage, and it's yours for the taking.

But do me a favor first. Stop and think. Really *think* about why you think you need this horrible code.
Is it worth the weight on your conscience? The cost to your soul?

This code is not fit for for production use, for demonstration purposes, or even for hobby projects.

Turn back ye wayward vagrant; thou shalt find no salvation here.
