use proconio::input;

fn main() {
    input! {
        n: i32
    }
    for i in n..=919 {
        let a = i / 100;
        let b = (i / 10) % 10;
        let c = i % 10;
        if a * b == c {
            println!("{}", i);
            break;
        }
    }
}
