use proconio::input;

fn main() {
    let mut a = vec![];
    loop {
        input! { n: u64 }
        a.push(n);
        if n == 0 {
            break;
        }
    }
    a.reverse();
    for a in &a {
        println!("{}", a);
    }
}
