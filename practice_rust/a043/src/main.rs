use proconio::input;
// use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; 5]
    }
    let mut ss: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        ss.push(
            (0..5)
                .map(|x| s[x].chars().nth(i).unwrap())
                .map(char_to_i)
                .collect(),
        );
    }

    let mut dp = vec![[0; 3]; n];

    let calc_c = |n: usize, c: usize| ss[n].iter().filter(|&x| x != &c).count() as i32;

    for c in 0..3 {
        dp[0][c] = calc_c(0, c);
    }

    for i in 1..n {
        for j in 0..3 {
            let c = calc_c(i, j);
            dp[i][j] = (0..3)
                .map(|k| {
                    if j == k {
                        std::i32::MAX
                    } else {
                        dp[i - 1][k] + c
                    }
                })
                .min()
                .unwrap();
        }
    }
    println!("{}", dp[n - 1].iter().min().unwrap());
}

fn char_to_i(c: char) -> usize {
    match c {
        'R' => 0,
        'B' => 1,
        'W' => 2,
        _ => 3,
    }
}

/*
/**
 * using chars
 */
fn main() {
    input! {
        n: usize,
        s: [String; 5]
    }
    let mut ss: Vec<Vec<char>> = vec![];
    for i in 0..n {
        ss.push((0..5).map(|x| s[x].chars().nth(i).unwrap()).collect());
    }

    let mut dp = vec![HashMap::from([('R', 0 as i32), ('B', 0 as i32), ('W', 0 as i32)]); n];

    let calc_c = |n: usize, c: char| ss[n].iter().filter(|&x| x != &c).count() as i32;

    for &c in &['R', 'B', 'W'] {
        dp[0].insert(c, calc_c(0, c));
    }

    for i in 1..n {
        for &j in &['R', 'B', 'W'] {
            let mut cost = std::i32::MAX;
            for &k in &['R', 'B', 'W'] {
                if k == j {
                    continue;
                }
                let c = calc_c(i, j);
                cost = cost.min(dp[i - 1].get(&k).unwrap() + c);
            }
            dp[i].insert(j, cost);
        }
    }
    println!("{}", dp[n - 1].values().min().unwrap());
}
*/
