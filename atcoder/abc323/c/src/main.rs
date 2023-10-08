use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; m],
        s: [Chars; n]
    }
    let mut points = vec![0_i32; n];
    for i in 0..n {
        points[i] += i as i32 + 1;
        for j in 0..m {
            if s[i][j] == 'o' {
                points[i] += a[j];
            }
        }
    }

    let max = *points.iter().max().unwrap();
    let mut ai = (0..m).collect::<Vec<_>>();
    ai.sort_by(|&x, &y| a[y].cmp(&a[x]));

    for i in 0..n {
        let mut remain = max - points[i];
        let mut cnt = 0;
        for &j in &ai {
            if s[i][j] == 'x' {
                if remain > a[j] {
                    remain -= a[j];
                    cnt += 1;
                } else {
                    println!("{}", if remain == 0 { cnt } else { cnt + 1 });
                    break;
                }
            }
        }
    }
}
