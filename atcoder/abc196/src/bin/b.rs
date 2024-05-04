use proconio::input;

fn main() {
    input! {
        x: String
    }
    let dot = x.find('.').unwrap_or(x.len());
    println!("{}", &x[0..dot])
}
