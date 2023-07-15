//! Coins
//!
//! https://atcoder.jp/contests/abc087/tasks/abc087_b

use std::cmp;

use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    };

    let mut count = 0;

    for a_count in 0..=cmp::min(x / 500, a) {
        // x - a_count * 500
        for b_count in 0..=cmp::min((x - a_count * 500) / 100, b) {
            if c >= (x - a_count * 500 - b_count * 100) / 50 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
