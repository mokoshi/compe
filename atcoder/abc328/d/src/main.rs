use proconio::input;

fn main() {
    input! {
        mut s: String
    }
    let mut t = String::new();
    for c in s.chars() {
        if c == 'A' || c == 'B' {
            t.push(c);
        } else {
            let l = t.len();
            if l >= 2 && &t[l - 2..l] == "AB" {
                t.remove(l - 1);
                t.remove(l - 2);
            } else {
                t.push(c);
            }
        }
    }
    println!("{}", t);
}
