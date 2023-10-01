use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
        t: String
    }
    let is_prefix = t.find(&s) == Some(0);
    let is_suffix = t
        .chars()
        .rev()
        .collect::<String>()
        .find(&(s.chars().rev().collect::<String>()))
        == Some(0);
    println!(
        "{}",
        if is_prefix && is_suffix {
            0
        } else if is_prefix {
            1
        } else if is_suffix {
            2
        } else {
            3
        }
    );
}
