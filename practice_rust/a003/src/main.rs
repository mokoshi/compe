use proconio::input;

// https://atcoder.jp/contests/abc122/tasks/abc122_b
fn main() {
    input! {
        n: String
    }

    let mut answer = 0;
    let mut current = 0;
    for c in n.chars() {
        if c == 'A' || c == 'C' || c == 'G' || c == 'T' {
            current += 1;
        } else {
            answer = answer.max(current);
            current = 0;
        }
    }
    answer = answer.max(current);

    println!("{}", answer)
}
