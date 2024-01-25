use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [u64; n],
        q: [u64; q]
    }
    r.sort();
    let mut acc = vec![r[0]; n];
    for i in 1..n {
        acc[i] = acc[i - 1] + r[i]
    }
    for &x in &q {
        let p = acc.partition_point(|&a| a <= x);
        println!("{}", p);
    }
}
