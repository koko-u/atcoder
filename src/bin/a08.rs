//! # Card Game for Two
//!
//! https://atcoder.jp/contests/abc088/tasks/abc088_b

use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    };

    a.sort_by_key(|n| cmp::Reverse(*n));

    let (alice_points, bob_points) =
        a.chunks(2)
            .fold((0, 0), |(alice_points, bob_points), chunk| match chunk {
                [alice, bob] => (alice_points + alice, bob_points + bob),
                [alice] => (alice_points + alice, bob_points),
                _ => (alice_points, bob_points),
            });

    println!("{}", alice_points - bob_points);
}
