use proconio::input;

// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d
fn main() {
    input! {
        _n: i32,
        s: String,
    };

    let mut answer = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let mut ok = 0;
                for c in s.chars() {
                    let cc = c.to_digit(10).unwrap();
                    if ok == 0 && cc == i {
                        ok += 1;
                    } else if ok == 1 && cc == j {
                        ok += 1;
                    } else if ok == 2 && cc == k {
                        answer += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", answer);
}
