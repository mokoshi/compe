use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    if s[0] != '<' || s[s.len() - 1] != '>' {
        println!("No");
        return;
    }
    for i in 1..s.len() - 1 {
        if s[i] != '=' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
