use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let uc = s.iter().filter(|&c| c.is_uppercase()).count();
    let lc = s.len() - uc;
    println!(
        "{}",
        if uc > lc {
            s.iter().map(|&c| c.to_uppercase()).join("")
        } else {
            s.iter().map(|&c| c.to_lowercase()).join("")
        }
    )
}
