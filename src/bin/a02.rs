//! # multiplication
//!
//! https://atcoder.jp/contests/abc169/tasks/abc169_a

use proconio::input;
fn main() {
    input! {
        ab: [i32; 2]
    };

    println!("{}", ab[0] * ab[1]);
}
