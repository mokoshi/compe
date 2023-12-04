use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32
    }
    if x < y {
        println!("{}", if y - x > 2 { "No" } else { "Yes" })
    } else {
        println!("{}", if x - y > 3 { "No" } else { "Yes" })
    }
}
