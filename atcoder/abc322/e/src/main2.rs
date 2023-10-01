use proconio::input;

const BIG: i64 = 10000000000000;
fn main() {
    input! {
        n: usize,
        k: i32,
        p: usize,
        cao: [(i64, [i32; k]); n]
    }
    let mut dp = vec![vec![vec![vec![vec![vec![BIG; 6]; 6]; 6]; 6]; 6]; n];
    for i in 0..n {
        dp[i][0][0][0][0][0] = 0;
    }

    let mut ca = vec![];
    for (c, ao) in cao {
        let mut a = vec![0; 5];
        for i in 0..ao.len() {
            a[i] = ao[i];
        }
        ca.push((c, a));
    }

    for i in 0..n {
        let (c, a) = &ca[i];
        if i == 0 {
            for [kp1, kp2, kp3, kp4, kp5] in make_patterns(a) {
                dp[0][kp1][kp2][kp3][kp4][kp5] = dp[0][kp1][kp2][kp3][kp4][kp5].min(*c);
            }
            continue;
        }
        for kp1 in 0..=5 {
            for kp2 in 0..=5 {
                for kp3 in 0..=5 {
                    for kp4 in 0..=5 {
                        for kp5 in 0..=5 {
                            let current = vec![kp1, kp2, kp3, kp4, kp5];
                            let prev = vec![
                                (kp1 - a[0]).max(0),
                                (kp2 - a[1]).max(0),
                                (kp3 - a[2]).max(0),
                                (kp4 - a[3]).max(0),
                                (kp5 - a[4]).max(0),
                            ];
                            let cost = dp[i - 1][kp1 as usize][kp2 as usize][kp3 as usize]
                                [kp4 as usize][kp5 as usize]
                                .min(
                                    c + dp[i - 1][prev[0] as usize][prev[1] as usize]
                                        [prev[2] as usize]
                                        [prev[3] as usize]
                                        [prev[4] as usize],
                                );
                            for [kp1, kp2, kp3, kp4, kp5] in make_patterns(&current) {
                                dp[i][kp1][kp2][kp3][kp4][kp5] =
                                    dp[i][kp1][kp2][kp3][kp4][kp5].min(cost);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut ans = BIG;
    for kp1 in if k < 1 { 0..=0 } else { p..=5 } {
        for kp2 in if k < 2 { 0..=0 } else { p..=5 } {
            for kp3 in if k < 3 { 0..=0 } else { p..=5 } {
                for kp4 in if k < 4 { 0..=0 } else { p..=5 } {
                    for kp5 in if k < 5 { 0..=0 } else { p..=5 } {
                        ans = ans.min(dp[n - 1][kp1][kp2][kp3][kp4][kp5]);
                    }
                }
            }
        }
    }
    println!("{}", if ans == BIG { -1 } else { ans });
}

fn make_patterns(a: &Vec<i32>) -> Vec<[usize; 5]> {
    let mut patterns =
        vec![[0; 5]; ((a[0] + 1) * (a[1] + 1) * (a[2] + 1) * (a[3] + 1) * (a[4] + 1)) as usize];
    let mut i = 0;
    for k1p in 0..=a[0] {
        for k2p in 0..=a[1] {
            for k3p in 0..=a[2] {
                for k4p in 0..=a[3] {
                    for k5p in 0..=a[4] {
                        patterns[i] = [
                            k1p as usize,
                            k2p as usize,
                            k3p as usize,
                            k4p as usize,
                            k5p as usize,
                        ];
                        i += 1;
                    }
                }
            }
        }
    }
    return patterns;
}
