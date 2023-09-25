fn main() {
    loop {
        let a = read_line();
        let w = a[0] as usize;
        let h = a[1] as usize;
        if w == 0 && h == 0 {
            break;
        }
        let mut c = vec![vec![]; h];
        for i in 0..h {
            c[i] = read_line();
        }

        let mut cnt = 0;
        let mut table = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                if c[i][j] == 1 && table[i][j] == 0 {
                    cnt += 1;
                    dfs((i, j), &c, &mut table, w, h);
                }
            }
        }
        println!("{}", cnt);
    }
}

fn dfs(p: (usize, usize), c: &Vec<Vec<i32>>, table: &mut Vec<Vec<i32>>, w: usize, h: usize) {
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let np = (p.0 as i32 + dy, p.1 as i32 + dx);
            if np.1 < 0 || np.1 > w as i32 - 1 || np.0 < 0 || np.0 > h as i32 - 1 {
                continue;
            }
            let up = (np.0 as usize, np.1 as usize);
            if table[up.0][up.1] == 0 && c[up.0][up.1] == 1 {
                table[up.0][up.1] = 1;
                dfs(up, c, table, w, h);
            }
        }
    }
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
