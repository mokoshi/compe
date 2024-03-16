use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    for i in 0..10 {
        if i != a + b {
            println!("{}", i);
            return;
        }
    }
}
