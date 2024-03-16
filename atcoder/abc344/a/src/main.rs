use proconio::input;

fn main() {
    input! {
        s: String
    }
    let a = s.find('|').unwrap();
    let b = s.rfind('|').unwrap();
    println!("{}{}", &s[..a], &s[b + 1..])
}
