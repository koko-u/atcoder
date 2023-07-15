//! # Shift only
//!

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let count = a.into_iter().map(|a| power_of_two(a, 0)).min().unwrap_or(0);
    println!("{}", count);
}

fn power_of_two(n: u32, power: usize) -> usize {
    if n % 2 == 1 {
        return power;
    }
    power_of_two(n / 2, power + 1)
}
