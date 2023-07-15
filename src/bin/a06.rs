//! # Some sums
//!

use text_io::read;
fn main() {
    let n: u32 = read!();
    let a: u32 = read!();
    let b: u32 = read!();

    let sum: u32 = (1..=n)
        .filter(|i| {
            let sum = sum_of_digits(*i, 0);
            sum >= a && sum <= b
        })
        .sum();

    println!("{}", sum);
}

fn sum_of_digits(n: u32, sum: u32) -> u32 {
    if n == 0 {
        return sum;
    }
    let q = n / 10;
    let r = n % 10;
    sum_of_digits(q, sum + r)
}
