use std::io;

fn main() {
    let mut c = vec![];
    for i in 1..=180 {
        c.push((181 - i) * (182 - i) * (183 - i) / 6);
    }

    let mut dp1 = vec![0; 1000000];
    let mut dp2 = vec![0; 1000000];
    for i in 1..1000000 {
        let mut m1 = std::i32::MAX;
        let mut m2 = std::i32::MAX;
        for j in 1..=180 {
            let jj = j * (j + 1) * (j + 2) / 6;
            if i == jj {
                m1 = 1;
                break;
            }
            if i < jj {
                break;
            }
            m1 = m1.min(dp1[i - jj] + 1);
        }
        for j2 in 0..=90 {
            let j = j2 * 2 + 1;
            let jj = j * (j + 1) * (j + 2) / 6;
            if jj % 2 == 0 {
                continue;
            }
            if i == jj {
                m2 = 1;
                break;
            }
            if i < jj {
                break;
            }
            m2 = m2.min(dp2[i - jj] + 1);
        }
        dp1[i] = m1;
        dp2[i] = m2;
    }

    loop {
        let a = read_line()[0];
        if a == 0 {
            break;
        }
        println!("{} {}", dp1[a], dp2[a]);
    }
}

fn read_line() -> Vec<usize> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
