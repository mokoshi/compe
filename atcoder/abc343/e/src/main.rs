use proconio::input;

fn main() {
    input! {
        v1: i32,
        v2: i32,
        v3: i32
    }
    for i in 1..=7 {
        for j in i..=7 {
            for k in j..=7 {
                if i * j * k != v3 {
                    continue;
                }
                let (y1, z1) = (0, 0);
                let (x2, z2) = (i - 7, k - 7);
                let (x3, y3) = (0, j - 7);
                for x1 in i - 7..=0 {
                    for y2 in j - 7..=0 {
                        for z3 in k - 7..=0 {
                            let mut cnt = vec![0; 4];
                            for a in x1.min(x2).min(x3)..=x1.max(x2).max(x3) + 7 {
                                for b in y1.min(y2).min(y3)..=y1.max(y2).max(y3) + 7 {
                                    for c in z1.min(z2).min(z3)..=z1.max(z2).max(z3) + 7 {
                                        let m = (col((a, b, c), (x1, y1, z1))
                                            + col((a, b, c), (x2, y2, z2))
                                            + col((a, b, c), (x3, y3, z3)))
                                            as usize;
                                        cnt[m] += 1;
                                    }
                                }
                            }
                            if cnt[1] == v1 && cnt[2] == v2 {
                                println!("Yes");
                                println!(
                                    "{} {} {} {} {} {} {} {} {}",
                                    x1, y1, z1, x2, y2, z2, x3, y3, z3
                                );
                                return;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}

fn col((x, y, z): (i32, i32, i32), (a, b, c): (i32, i32, i32)) -> i32 {
    if x >= a && x < a + 7 && y >= b && y < b + 7 && z >= c && z < c + 7 {
        1
    } else {
        0
    }
}
