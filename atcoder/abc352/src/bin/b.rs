use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars
    }
    let mut c = 0;
    for i in 0..s.len() {
        loop {
            if s[i] == t[c] {
                print!("{} ", c + 1);
                c += 1;
                break;
            }
            c += 1;
        }
    }
    println!()
}
