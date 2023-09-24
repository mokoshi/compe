fn main() {
    let n = read_line()[0] as usize;
    let mut g = vec![vec![]; n + 1];
    for _ in 0..n {
        let a = read_line();
        let k = a[0];
        let u = a[1];
        for j in 0..u {
            g[k as usize].push(a[j as usize + 2]);
        }
    }
    for i in 1..=n {
        g[0].push(i as i32);
    }

    let mut dt = vec![0; n + 1];
    let mut ft = vec![0; n + 1];
    dt[0] = 0;

    dfs(0, 0, &g, &mut dt, &mut ft);

    for i in 1..=n {
        println!("{} {} {}", i, dt[i], ft[i]);
    }
}

fn dfs(p: i32, d: i32, g: &Vec<Vec<i32>>, dt: &mut Vec<i32>, ft: &mut Vec<i32>) -> i32 {
    dt[p as usize] = d;

    let mut c = d;
    for np in &g[p as usize] {
        if dt[*np as usize] == 0 {
            c = dfs(*np, c + 1, g, dt, ft);
        }
    }
    ft[p as usize] = c + 1;
    return c + 1;
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
