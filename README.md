# monkeys-rs
A rust library project to simulate monkeys typing random ASCII characters to form words.

# Example
This example outputs the averaged number of times the monkey has had to type two random ASCII character to reach the desired results specified in the constructor.
```rust
use monkeys_rs::Monkey;

fn main() {
    let iterations = 100;
    let mut monkey = Monkey::new("Hi");
    let mut total_attempts = 0;

    (0..iterations).into_iter()
            .for_each(|_| total_attempts += monkey.compute_attempts());
    println!("{}", total_attempts/iterations);
}
```
