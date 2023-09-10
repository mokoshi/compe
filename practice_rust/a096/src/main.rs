use proconio::input;

// https://atcoder.jp/contests/abc139/tasks/abc139_d
fn main() {
    input! {
        n: i64
    }
    println!("{}", n * (n - 1) / 2);
}
