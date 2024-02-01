use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }
    let len = s.len();
    s[len - 1] = '4';
    println!("{}", s.into_iter().collect::<String>());
}
