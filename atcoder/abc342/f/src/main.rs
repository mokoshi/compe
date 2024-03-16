use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        d: usize,
    }
    let df = d as f64;
    let dd = 1.0 / df;
    let mut v = vec![0_f64; n + 1];
    let mut av = 0_f64;
    let mut p = vec![0_f64; n + 1];
    p[0] = 1.0;
    for i in 1..=n {
        let e = i + d;
        let pd = p[i - 1] / df;
        v[i] += pd;
        if e <= n {
            v[e] -= pd;
        }
        av += v[i];
        p[i] = av;
    }
    println!("{:?}", p);
}
