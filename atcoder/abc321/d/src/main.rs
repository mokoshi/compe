use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        p: i64,
        mut a: [i64; n],
        mut b: [i64; m],
    }
    a.sort();
    b.sort();

    let mut bt = vec![0; m];
    bt[0] = b[0];
    for i in 1..m {
        bt[i] = bt[i - 1] + b[i];
    }

    let mut ans = 0;
    for i in 0..n {
        let aa = a[i];
        if aa >= p {
            ans += p * m as i64;
            continue;
        }
        let mi = b.partition_point(|&x| x < p - aa);
        if mi == 0 {
            ans += p * m as i64;
        } else {
            ans += aa * (mi as i64) + bt[mi - 1] + (m - mi) as i64 * p;
        }
    }
    println!("{}", ans);
}
