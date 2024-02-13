use proconio::input;

fn main() {
    input! {
        s: String
    }
    let p = s.rfind('.').unwrap();
    println!("{}", &s[p + 1..])
}
