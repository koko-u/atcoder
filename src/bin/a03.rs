//! # Product
//!
//! https://atcoder.jp/contests/abc086/tasks/abc086_a

use text_io::read;

fn main() {
    let a: i32 = read!();
    let b: i32 = read!();

    let answer = if a % 2 == 0 || b % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };
    println!("{}", answer);
}
