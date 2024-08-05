use proconio::{input, marker::Chars};

fn main() {
    input! {
        rms: Chars
    }
    for c in rms {
        if c == 'R' {
            println!("Yes");
            break;
        } else if c == 'M' {
            println!("No");
            break;
        }
    }
}
