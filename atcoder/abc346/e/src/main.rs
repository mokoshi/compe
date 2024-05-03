use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        tax: [(u8, usize, usize); m]
    }
    let mx = 200001;
    let mut rows = vec![mx; h];
    let mut cols = vec![mx; w];
    let mut rc = 0;
    let mut cc = 0;
    let mut c = vec![0; mx];
    for i in 0..m {
        let (t, a, x) = tax[m - 1 - i];
        if t == 1 {
            if rows[a - 1] == mx {
                rows[a - 1] = x;
                c[x] += w - cc;
                rc += 1;
            }
        } else {
            if cols[a - 1] == mx {
                cols[a - 1] = x;
                c[x] += h - rc;
                cc += 1;
            }
        }
    }
    let cs = c.iter().sum::<usize>() - c[0];
    c[0] = w * h - cs;
    let ans: Vec<(usize, &usize)> = c.iter().enumerate().filter(|&(_, &c)| c > 0).collect();
    println!("{}", ans.len());
    for (x, &c) in ans {
        println!("{} {}", x, c);
    }
}
