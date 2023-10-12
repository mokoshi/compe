use proconio::{input, marker::Chars};

fn main() {
    input! {m: usize, s: [Chars; 3]}

    let mut ans = std::usize::MAX;
    for reel in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        for x in 0..10 {
            let c = x.to_string().chars().nth(0).unwrap();
            let mut reeli = 0;
            for t in 0..3 * m {
                let tc = s[reel[reeli]][t % m];
                if tc == c {
                    if reeli == 2 {
                        ans = ans.min(t);
                        break;
                    }
                    reeli += 1;
                }
            }
        }
    }
    println!(
        "{}",
        if ans == std::usize::MAX {
            -1
        } else {
            ans as i32
        }
    );
}
