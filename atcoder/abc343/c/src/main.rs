use proconio::input;

fn main() {
    input! {
        n: u64
    }
    let mut ans = 0;
    for i in 0..=1000000 {
        let v = i * i * i;
        if v > n {
            break;
        }
        let s = v.to_string().chars().collect::<Vec<char>>();
        let mut ok = true;
        for j in 0..s.len() / 2 {
            if s[j] != s[s.len() - 1 - j] {
                ok = false;
                break;
            }
        }
        if ok {
            ans = v;
        }
    }
    println!("{}", ans);
}
