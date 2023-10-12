use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        p: i32
    }
    if n < m {
        println!("0");
    } else {
        println!("{}", (n - m) / p + 1);
    }
}
