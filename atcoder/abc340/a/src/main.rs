use proconio::input;

fn main() {
    input! {
        mut a: i32,
        b: i32,
        d: i32
    }
    while a <= b {
        print!("{} ", a);
        a += d;
    }
    println!()
}
