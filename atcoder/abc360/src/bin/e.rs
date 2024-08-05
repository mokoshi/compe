use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: Mint,
        k: usize
    }
    let p = ((n - 1).pow(2) + 1) / (n * n);
    let q = Mint::new(2) / (n * n);
    let mut s = Mint::new(1);
    for _ in 0..k {
        let t = -s + 1;
        s = s * p + t * q;
    }
    println!("{}", s + (-s + 1) * (n + 2) / 2);
}
