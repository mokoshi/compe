use proconio::input;

fn main() {
    input! {
        x: i128
    }
    if x < 0 {
        println!("{}", x / 10);
    } else {
        println!("{}", if x % 10 == 0 { x / 10 } else { x / 10 + 1 });
    }
}
