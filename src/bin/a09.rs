//! AcCepted
//!
//! https://atcoder.jp/contests/abc104/tasks/abc104_b

use text_io::read;

fn main() {
    let s: String = read!();
    let len = s.len();

    let mut answer: &str = "AC";
    let mut c_count = 0;
    for (idx, c) in s.char_indices() {
        match idx {
            0 => {
                if c != 'A' {
                    answer = "WA";
                    break;
                }
            }
            _ if idx >= 2 && idx <= len - 2 => {
                if c == 'C' {
                    c_count += 1;
                } else if !c.is_ascii_lowercase() {
                    answer = "WA";
                    break;
                }
            }
            _ => {
                if !c.is_ascii_lowercase() {
                    answer = "WA";
                    break;
                }
            }
        }
    }
    if c_count != 1 {
        answer = "WA";
    }

    println!("{}", answer);
}
