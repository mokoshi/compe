use proconio::input;

// https://atcoder.jp/contests/abc149/tasks/abc149_b
fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64,
    }
    println!(
        "{} {}",
        if a >= k { a - k } else { 0 },
        if a >= k { b } else { b - (k - a) }
    );
}
