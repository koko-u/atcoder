//! # We Love Golf
//!
//! https://atcoder.jp/contests/abc165/tasks/abc165_a

use proconio::input;

fn main() {
    input! {
        k: u32,
        range: [u32; 2],
    };

    // a <= k * n <= b
    // a / k <= n <= b / k
    // cell(a / k) <= floor(b / k)

    let upper = (range[1] as f64 / k as f64).floor();
    let lower = (range[0] as f64 / k as f64).ceil();

    let answer = if upper >= lower { "OK" } else { "NG" };

    println!("{}", answer);
}
