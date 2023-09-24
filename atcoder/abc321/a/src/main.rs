use proconio::input;

fn main() {
    input! {
        n: String
    }

    let mut prev = 10;
    for c in n.chars() {
        let d = c.to_digit(10).unwrap();
        if d >= prev {
            println!("No");
            return;
        }
        prev = d;
    }
    println!("Yes");
}
