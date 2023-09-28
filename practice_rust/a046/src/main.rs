fn main() {
    let n = read_line()[0] as usize;
    let mut m = vec![];
    for _ in 0..n {
        let a = read_line();
        m.push((a[0], a[1]));
    }

    let mut dp = vec![vec![0; n]; n];
    for w in 1..n {
        for l in 0..n - w {
            let r = l + w;
            dp[l][r] = (l..r)
                .map(|x| {
                    let m1 = (m[l].0, m[x].1);
                    let m2 = (m[x + 1].0, m[r].1);
                    dp[l][x] + dp[x + 1][r] + cnt(m1, m2)
                })
                .min()
                .unwrap();
        }
    }
    println!("{:?}", dp[0][n - 1]);
}

fn cnt(m1: (i32, i32), m2: (i32, i32)) -> i32 {
    return m1.0 * m2.1 * m1.1;
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
