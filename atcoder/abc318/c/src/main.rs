use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        p: i64,
        mut f: [i64; n]
    }
    f.sort();

    let cp = p / d as i64;
    let mut cnt = 0;
    for i in 0..n {
        if f[i] > cp {
            cnt = n - i;
            break;
        }
    }
    let dn = cnt / d;

    let mut ans1 = dn as i64 * p;
    for i in dn * d..n {
        ans1 += f[n - 1 - i];
    }
    let mut ans2 = (dn as i64 + 1) * p;
    for i in (dn + 1) * d..n {
        ans2 += f[n - 1 - i];
    }
    println!("{}", ans1.min(ans2));
}
