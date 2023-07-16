use proconio::input;

fn main() {
    input! {
        n: usize,
        _m: usize
    }

    let mut p = vec![0; n];
    let mut f = vec![vec![]; n];
    for i in 0..n {
        input! {
            pi: usize,
            ci: usize,
            fi: [usize; ci]
        }
        p[i] = pi;
        f[i] = fi;
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if p[i] >= p[j] {
                let mut flag = true;
                for k in 0..f[i].len() {
                    if !f[j].contains(&f[i][k]) {
                        flag = false;
                        break;
                    }
                }
                if p[i] == p[j] && f[i].len() == f[j].len() {
                    flag = false;
                }
                if flag {
                    // println!("{} {}", i + 1, j + 1);
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
