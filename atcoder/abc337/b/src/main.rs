use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    println!("{}", if judge(&s) { "Yes" } else { "No" })
}

fn judge(s: &Vec<char>) -> bool {
    let mut a = 0;
    for &c in s {
        if a == 0 {
            if c == 'B' {
                a += 1;
            } else if c == 'C' {
                a += 2;
            } else if c != 'A' {
                return false;
            }
        } else if a == 1 {
            if c == 'C' {
                a += 1;
            } else if c != 'B' {
                return false;
            }
        } else {
            if c != 'C' {
                return false;
            }
        }
    }
    true
}
