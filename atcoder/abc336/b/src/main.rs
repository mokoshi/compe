use proconio::input;

fn main() {
    input! {
        n: usize
    }
    for i in 0..=30 {
        if n % 2_usize.pow((30 - i) as u32) == 0 {
            println!("{}", 30 - i);
            break;
        }
    }
}
