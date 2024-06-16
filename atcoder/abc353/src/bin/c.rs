use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n]
    }
    let k = 100_000_000;
    a.sort();
    let mut overc = 0;
    for i in 0..n - 1 {
        let p = a[i + 1..].partition_point(|&x| x < k - a[i]);
        overc += n - (p + i + 1);
    }
    println!(
        "{}",
        a.iter().sum::<u64>() * ((n - 1) as u64) - k * (overc as u64)
    );
}
