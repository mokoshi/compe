use std::{cmp::min, io, vec};

fn main() {
    loop {
        let a = read_line();
        let n = a[0] as usize;
        let m = a[1] as usize;
        if n == 0 && m == 0 {
            break;
        }

        let mut cc = vec![];
        for _ in 0..m {
            cc.push(read_line()[0]);
        }
        let mut xx = vec![];
        for _ in 0..n {
            xx.push(read_line()[0]);
        }

        let maxv = std::i32::MAX - 255 * 255;

        let mut dp = vec![vec![maxv; 256]; n + 1];
        dp[0][128] = 0;

        for i in 0..n {
            for j in 0..256 {
                for k in 0..m {
                    if dp[i][j] < maxv {
                        let c = cc[k];
                        let nj = (j as i32 + c).max(0).min(255) as usize;
                        dp[i + 1][nj] = min(dp[i + 1][nj], dp[i][j] + (nj as i32 - xx[i]).pow(2));
                    }
                }
            }
        }

        /* n を計算するために n-1 を参照するロジック。0 ~ 255 で丸められるため、ちょっと厄介
               for i in 1..n + 1 {
                   for j in 0..256 {
                       let mut minc = maxv;
                       for k in 0..m {
                           let c = cc[k];
                           let range;
                           if j == 0 {
                               if c > 0 {
                                   continue;
                               }
                               range = 0..=j - c;
                           } else if j == 255 {
                               if c < 0 {
                                   continue;
                               }
                               range = j - c..=255;
                           } else {
                               if j - c < 0 || j - c > 255 {
                                   continue;
                               }
                               range = j - c..=j - c;
                           }
                           minc = minc.min(
                               range
                                   .map(|x| dp[i - 1][x as usize] + (j as i32 - xx[i - 1]).pow(2))
                                   .min()
                                   .unwrap(),
                           )
                       }
                       dp[i][j as usize] = minc;
                   }
               }
        */

        println!("{}", dp[n].iter().min().unwrap_or(&0));
    }
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
