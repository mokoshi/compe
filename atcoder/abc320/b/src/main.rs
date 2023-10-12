use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut ans = 0;
    for i in 0..=s.len() {
        for j in i + 1..=s.len() {
            if is_kai(&s[i..j]) {
                ans = ans.max(j - i);
            }
        }
    }
    println!("{}", ans)
}

fn is_kai(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
    }
    return true;
}
