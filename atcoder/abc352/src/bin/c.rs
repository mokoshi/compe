use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n]
    }
    let mut hi = 0;
    let mut h = ab[0].1 - ab[0].0;
    for i in 1..n {
        if h < ab[i].1 - ab[i].0 {
            hi = i;
            h = ab[i].1 - ab[i].0;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if i == hi {
            ans += ab[i].1;
        } else {
            ans += ab[i].0;
        }
    }
    println!("{}", ans);
}
