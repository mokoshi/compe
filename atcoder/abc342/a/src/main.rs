use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    if s[0] == s[1] && s[1] == s[2] {
        for i in 0..s.len() {
            if s[i] != s[0] {
                println!("{}", i + 1);
                return;
            }
        }
    } else if s[0] == s[1] && s[1] != s[2] {
        println!("3")
    } else if s[0] != s[1] && s[1] == s[2] {
        println!("1")
    } else {
        println!("2")
    }
}
