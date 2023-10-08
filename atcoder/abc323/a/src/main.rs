use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    for i in 1..=8 {
        if s[i * 2 - 1] == '1' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
