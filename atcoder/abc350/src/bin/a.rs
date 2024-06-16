use proconio::input;

fn main() {
    input! {
        s: String
    }
    let abc = &s[0..3];
    let num = s[3..6].parse::<u32>().unwrap();
    if abc == "ABC" && num < 350 && num > 0 && num != 316 {
        println!("Yes");
    } else {
        println!("No");
    }
}
