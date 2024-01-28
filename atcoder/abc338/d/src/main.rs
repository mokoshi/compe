use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        x: [usize; m],
    }
    let mut dv = vec![0_i64; n as usize];
    for i in 0..m - 1 {
        let s = x[i].min(x[i + 1]) - 1;
        let e = x[i].max(x[i + 1]) - 1;
        let d = (e - s) as i64;
        dv[s] += n - d - d;
        dv[e] += d - (n - d);
        dv[0] += d;
        // println!("s: {}, e: {}, d: {}", s, e, d);
        // println!("{:?}", dv);
    }

    let mut c = vec![0_i64; n as usize];
    c[0] = dv[0];
    for i in 1..n as usize {
        c[i] = c[i - 1] + dv[i];
    }
    // println!("{:?}", c);
    println!("{}", c.iter().min().unwrap());
}
