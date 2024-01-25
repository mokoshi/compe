use proconio::input;

fn main() {
    input! {
        b: i32,
        g: i32
    }
    println!(
        "{}",
        match b > g {
            true => "Bat",
            false => "Glove",
        }
    );
}
