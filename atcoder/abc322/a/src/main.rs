use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }
    let a = s.find("ABC");
    println!("{}", if a == None { -1 } else { a.unwrap() as i32 + 1 });
}
