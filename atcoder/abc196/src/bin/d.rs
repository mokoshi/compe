use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        _b: usize
    }
    if a == 0 {
        println!("1");
        return;
    }
    let mut ptns = 0;
    for p in (0..h * w).combinations(a) {
        let b = vec![vec![false; w]; h];
        ptns += dfs(0, &p, &b, h, w);
    }
    println!("{}", ptns);
}

fn dfs(i: usize, positions: &Vec<usize>, b: &Vec<Vec<bool>>, h: usize, w: usize) -> usize {
    let mut c = 0;
    for hor in [false, true] {
        let mut b = b.clone();
        if try_put(positions[i] % w, positions[i] / w, hor, &mut b, h, w) {
            c += if i == positions.len() - 1 {
                1
            } else {
                dfs(i + 1, positions, &b, h, w)
            };
        }
    }
    return c;
}

fn try_put(x: usize, y: usize, hor: bool, b: &mut Vec<Vec<bool>>, h: usize, w: usize) -> bool {
    if hor {
        if x + 1 < w && b[y][x] == false && b[y][x + 1] == false {
            b[y][x] = true;
            b[y][x + 1] = true;
            return true;
        } else {
            return false;
        };
    } else {
        if y + 1 < h && b[y][x] == false && b[y + 1][x] == false {
            b[y][x] = true;
            b[y + 1][x] = true;
            return true;
        } else {
            return false;
        };
    }
}
