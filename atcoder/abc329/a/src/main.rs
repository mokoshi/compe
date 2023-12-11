use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    println!(
        "{}",
        s.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}
