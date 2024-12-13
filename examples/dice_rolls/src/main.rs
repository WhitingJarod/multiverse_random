use multiverse_random::random;

fn main() {
    let dice_result = random(1..=4) + random(1..=4) + 2;
    if dice_result < 6 {
        panic!("fatal error: the result of 2d4+2 was not high enough. terminating.")
    }
    println!("The result of rolling 2d4+2 was {}", dice_result);
}
