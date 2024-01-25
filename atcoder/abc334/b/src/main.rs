use proconio::input;

fn main() {
    input! {
        a: i64,
        m: i64,
        mut l: i64,
        mut r: i64
    }
    println!("{}", (r - a).div_euclid(m) - (l - 1 - a).div_euclid(m));
}
