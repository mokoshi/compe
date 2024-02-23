use proconio::input;

fn main() {
    input! {
        n: usize,
        s: u32,
        k: u32,
        pq: [(u32, u32); n]
    }
    let mut ans = 0;
    for &(p, q) in &pq {
        ans += p * q;
    }
    if ans < s {
        ans += k;
    }
    println!("{}", ans)
}
