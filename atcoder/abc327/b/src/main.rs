use proconio::input;

fn main() {
    input! {
        b: u64
    }
    for i in 1..=15_u64 {
        let c = i.pow(i as u32);
        if c == b {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
