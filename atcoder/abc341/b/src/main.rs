use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        st: [(u64, u64); n - 1]
    }
    for i in 0..n - 1 {
        let c = a[i] / st[i].0;
        a[i + 1] += st[i].1 * c;
    }
    println!("{}", a[n - 1]);
}
