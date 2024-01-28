use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut ans = true;
    for i in 0..s.len() {
        if i == 0 {
            if !s[i].is_ascii_uppercase() {
                ans = false;
                break;
            }
        } else {
            if !s[i].is_ascii_lowercase() {
                ans = false;
                break;
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" })
}
