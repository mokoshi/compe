use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [u32; n]
    }
    for i in 0..n {
        if a[i] % k == 0 {
            print!("{} ", a[i] / k);
        }
    }
    println!();
}
