use proconio::input;

fn main() {
    input! {
        n: usize
    }
    for i in 0..=n * 2 {
        print!("{}", if i % 2 == 0 { 1 } else { 0 })
    }
    println!()
}
