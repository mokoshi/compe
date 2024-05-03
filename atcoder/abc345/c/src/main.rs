use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut t = vec![0; 26];
    for j in 0..s.len() {
        let i = s[j] as usize - 'a' as usize;
        t[i] += 1;
    }
    let mut d = 0;
    for i in 0..26 {
        if t[i] >= 2 {
            d += (t[i] * (t[i] - 1)) / 2;
        }
    }
    println!(
        "{:?}",
        (s.len() * (s.len() - 1)) / 2 - d + if d > 0 { 1 } else { 0 }
    );
}
