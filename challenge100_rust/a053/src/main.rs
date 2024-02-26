fn main() {
    let n = read_line()[0] as usize;
    let inf = 10000000000;
    let mut dp = vec![inf; n];
    for i in 0..n {
        let a = read_line()[0];
        let mut left = 0;
        let mut right = i;
        while left < right {
            let mid = (left + right) / 2;
            if dp[mid] < a {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        dp[left] = a;
    }
    for i in 0..n {
        if dp[i] == inf {
            println!("{}", i);
            return;
        }
    }
    println!("{}", n);
}

fn read_line() -> Vec<u64> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

/*
2
1
2

 */
