use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n]
    }
    for i in 0..n - 1 {
        println!("{}", a[i] * a[i + 1])
    }
}
