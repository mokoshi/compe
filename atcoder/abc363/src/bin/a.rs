use proconio::input;

fn main() {
    input! {
        r: u32
    }
    let a = (400 - r) % 100;
    println!("{}", if a == 0 { 100 } else { a });
}
