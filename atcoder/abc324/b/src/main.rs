use proconio::input;

fn main() {
    input! {
        mut n: i128
    }

    loop {
        if n % 2 == 0 {
            n = n / 2;
        } else {
            break;
        }
    }
    loop {
        if n % 3 == 0 {
            n = n / 3;
        } else {
            break;
        }
    }
    println!("{}", if n == 1 { "Yes" } else { "No" })
}
