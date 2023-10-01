use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut p1: [Chars; 4],
        mut p2: [Chars; 4],
        mut p3: [Chars; 4],
    }

    let mut total = 0;
    for i in 0..4 {
        for j in 0..4 {
            if p1[i][j] == '#' {
                total += 1;
            }
            if p2[i][j] == '#' {
                total += 1;
            }
            if p3[i][j] == '#' {
                total += 1;
            }
        }
    }
    if total != 16 {
        println!("No");
        return;
    }

    let p1s = all_ps(p1);
    let p2s = all_ps(p2);
    let p3s = all_ps(p3);

    let p = &p1s[0].0;
    let (l, r, t, b) = p1s[0].1;
    let mut tables = vec![];
    let mut table = vec![vec!['.'; 4]; 4];
    for dx in -l..=r {
        for dy in -t..=b {
            put_p(p, dx, dy, &mut table);
            tables.push(table);
            table = vec![vec!['.'; 4]; 4];
        }
    }
    // for p3si in &p3s {
    //     println!("{:?}", p3si);
    // }

    for t in tables {
        // println!("t1: {:?}", t);
        let tables2 = get_patterns(&t, &p2s);
        for t2 in tables2 {
            // println!("t2: {:?}", t2);
            let tables3 = get_patterns(&t2, &p3s);
            if tables3.len() > 0 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn get_patterns(
    origin: &Vec<Vec<char>>,
    ps: &Vec<(Vec<Vec<char>>, (i32, i32, i32, i32))>,
) -> Vec<Vec<Vec<char>>> {
    let mut tables = vec![];
    for i in 0..4 {
        let p = &ps[i].0;
        let (l, r, t, b) = ps[i].1;
        for dx in -l..=r {
            for dy in -t..=b {
                let mut table = origin.clone();
                if put_p(p, dx, dy, &mut table) {
                    tables.push(table);
                };
            }
        }
    }
    return tables;
}

fn put_p(p: &Vec<Vec<char>>, dx: i32, dy: i32, table: &mut Vec<Vec<char>>) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            let pxi = j as i32 - dx;
            let pyi = i as i32 - dy;
            if pxi < 0 || pxi > 3 || pyi < 0 || pyi > 3 {
                continue;
            }
            let px = pxi as usize;
            let py = pyi as usize;
            let c = p[py][px];
            if c == '.' {
                continue;
            } else {
                if table[i][j] == '#' {
                    return false;
                } else {
                    table[i][j] = '#';
                }
            }
        }
    }
    return true;
}

fn all_ps(p: Vec<Vec<char>>) -> Vec<(Vec<Vec<char>>, (i32, i32, i32, i32))> {
    let p1 = p.clone();
    let p2 = rot(&p1);
    let p3 = rot(&p2);
    let p4 = rot(&p3);
    let lrtb1 = lrtb(&p1);
    let lrtb2 = lrtb(&p2);
    let lrtb3 = lrtb(&p3);
    let lrtb4 = lrtb(&p4);
    return vec![(p1, lrtb1), (p2, lrtb2), (p3, lrtb3), (p4, lrtb4)];
}

fn rot(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rr = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            rr[i][j] = p[3 - j][i];
        }
    }
    return rr;
}

fn lrtb(p: &Vec<Vec<char>>) -> (i32, i32, i32, i32) {
    let mut l = -1;
    let mut r = -1;
    let mut t = -1;
    let mut b = -1;
    for i in 0..4 {
        for j in 0..4 {
            if l == -1 && p[j][i] == '#' {
                l = i as i32;
            }
            if r == -1 && p[j][3 - i] == '#' {
                r = i as i32;
            }
            if t == -1 && p[i][j] == '#' {
                t = i as i32;
            }
            if b == -1 && p[3 - i][j] == '#' {
                b = i as i32;
            }
        }
    }
    return (l, r, t, b);
}
