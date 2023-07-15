//! # Serval vs Monster
//!
//! https://atcoder.jp/contests/abc153/tasks/abc153_a

use text_io::read;
fn main() {
    let h: u32 = read!();
    let a: u32 = read!();

    let q = h / a;
    let r = h % a;
    let answer = if r == 0 { q } else { q + 1 };
    println!("{}", answer);
}
