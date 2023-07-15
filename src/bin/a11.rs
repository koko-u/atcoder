//! # Default Price
//!
//! https://atcoder.jp/contests/abc308/tasks/abc308_b
//!
//!

use std::collections::HashMap;
use text_io::read;

fn main() {
    let _n: usize = read!();
    let m: usize = read!();
    let dishes: String = read!("{}\n");
    let colors: String = read!("{}\n");
    let prices: String = read!("{}\n");

    let mut price_map = HashMap::<String, u32>::with_capacity(m + 1);
    let mut prices = prices
        .split_whitespace()
        .map(|price| price.parse::<u32>().unwrap());
    let default_price = prices.next().unwrap();

    for (color, price) in colors.split_whitespace().zip(prices) {
        price_map.insert(color.into(), price);
    }

    let mut amount = 0;
    for dish in dishes.split_whitespace() {
        amount += price_map.get(dish).unwrap_or(&default_price);
    }

    println!("{}", amount);
}
