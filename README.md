# Multiverse Random

**Multiverse Random** provides a single function
```rust
random<T, U>(items: U) -> T
where U: IntoIterator<Item = T>
```
This function takes some list or iterator over items of type T, selects one, and returns it.

## Examples
`random` is easy to use.
The most common use pattern is to generate numbers from a range.
```rust
use multiverse_random::random;

let dice_result = random(1..=4) + random(1..=4) + 2;
if dice_result < 6 {
    // crash the program here; we don't tolerate weak rollers at my table
    panic!("fatal error: the result of 2d4+2 was not high enough. terminating.")
}
println!("The result of rolling 2d4+2 was {}", dice_result);
```
The code above is straightforward. We generate two random numbers between 1 and 4 (inclusive), add 2,
and check if the result is less than 6.

As you'd expect, running the program prints 16 results. Three of those are panics.
```
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/dice_rolls`

thread 'main' panicked at src/main.rs:6:9:
fatal error: the result of 2d4+2 was not high enough. terminating.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

The result of rolling 2d4+2 was 7
The result of rolling 2d4+2 was 7
The result of rolling 2d4+2 was 8

thread 'main' panicked at src/main.rs:6:9:
fatal error: the result of 2d4+2 was not high enough. terminating.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

The result of rolling 2d4+2 was 7

thread 'main' panicked at src/main.rs:6:9:
fatal error: the result of 2d4+2 was not high enough. terminating.
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

The result of rolling 2d4+2 was 6
The result of rolling 2d4+2 was 6
The result of rolling 2d4+2 was 6
The result of rolling 2d4+2 was 7
The result of rolling 2d4+2 was 8
The result of rolling 2d4+2 was 9
The result of rolling 2d4+2 was 8
The result of rolling 2d4+2 was 9
```

Another common pattern is to choose a random item from a list.

```rust
use multiverse_random::random;

let strings = ["foo", "bar", "baz"]
let choice = random(strings);
println!("random string: {}");
```
As expected, running the code prints all three items in the list.
```
❯ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/string_test`

random string: foo
random string: baz
random string: bar
```



# License
The code is protected under the `Why Would You Ever Even Use This` standard license agreement.
This means that you are free to do literally anything with the code. Literally anything.

It's free, it's open source, it's garbage, and it's yours for the taking.

But do yourself a favor first. Stop and think. Really *think* about why you want to use this trash.
Is it worth the weight on your conscience? The cost to your soul?
