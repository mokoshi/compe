use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut ans = 0;
    let mut c = 0;
    for i in 0..n {
        if c + a[i] > k {
            c = a[i];
            ans += 1;
        } else {
            c += a[i];
        }
    }
    if c > 0 {
        ans += 1;
    }
    println!("{}", ans);
}
