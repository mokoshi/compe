use itertools::Itertools;
use proconio::{input, marker::Chars};

type Table = Vec<Vec<char>>;
type PolyoMino = Vec<Vec<char>>;

fn main() {
    input! {
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4],
    }
    let p2s = all_ps(&p2);
    let p3s = all_ps(&p3);

    if p1.iter().flatten().filter(|&x| x == &'#').count()
        + p2.iter().flatten().filter(|&x| x == &'#').count()
        + p3.iter().flatten().filter(|&x| x == &'#').count()
        != 16
    {
        println!("No");
        return;
    }

    let trans1 = get_translates(&p1);
    for p2 in &p2s {
        let trans2 = get_translates(&p2);
        for p3 in &p3s {
            let trans3 = get_translates(&p3);
            for &(dx1, dy1) in &trans1 {
                for &(dx2, dy2) in &trans2 {
                    for &(dx3, dy3) in &trans3 {
                        let mut table = vec![vec!['.'; 4]; 4];
                        if put_p(&p1, dx1, dy1, &mut table)
                            && put_p(&p2, dx2, dy2, &mut table)
                            && put_p(&p3, dx3, dy3, &mut table)
                        {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}

fn get_translates(p: &PolyoMino) -> Vec<(i32, i32)> {
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
    return (-l..=r).cartesian_product(-t..=b).collect_vec();
}

fn put_p(p: &PolyoMino, dx: i32, dy: i32, table: &mut Table) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            let c = p[i][j];
            if c == '.' {
                continue;
            }

            let bx = j as i32 + dx;
            let by = i as i32 + dy;
            if bx < 0 || bx > 3 || by < 0 || by > 3 {
                return false;
            }
            if table[bx as usize][by as usize] == '#' {
                return false;
            }
            table[bx as usize][by as usize] = '#';
        }
    }
    return true;
}

fn all_ps(p: &PolyoMino) -> Vec<PolyoMino> {
    let p1 = p.clone();
    let p2 = rot(&p1);
    let p3 = rot(&p2);
    let p4 = rot(&p3);
    return vec![p1, p2, p3, p4];
}

fn rot(p: &PolyoMino) -> PolyoMino {
    let mut rr = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            rr[i][j] = p[3 - j][i];
        }
    }
    return rr;
}
