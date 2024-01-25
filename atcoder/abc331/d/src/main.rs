use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        qq: usize,
        p: [Chars; n],
        q: [(usize, usize, usize, usize); qq]
    }
    let mut tbl = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let a = if p[i][j] == 'B' { 1 } else { 0 };
            if i == 0 && j == 0 {
                tbl[i][j] = a;
            } else if i == 0 {
                tbl[i][j] = tbl[i][j - 1] + a;
            } else if j == 0 {
                tbl[i][j] = tbl[i - 1][j] + a;
            } else {
                tbl[i][j] = tbl[i - 1][j] + tbl[i][j - 1] - tbl[i - 1][j - 1] + a;
            }
        }
    }
    println!("{:?}", tbl);
    for &(a, b, c, d) in &q {
        let tl = if a % n == 0 && b % n == 0 {
            tbl[n - 1][n - 1]
        } else if a % n == 0 {
            tbl[n - 1][n - 1] - tbl[n - 1][(b + n - 1) % n]
        } else if b % n == 0 {
            tbl[n - 1][n - 1] - tbl[(a + n - 1) % n][n - 1]
        } else {
            tbl[n - 1][n - 1] + tbl[(a + n - 1) % n][(b + n - 1) % n]
                - tbl[(a + n - 1) % n][n - 1]
                - tbl[n - 1][(b + n - 1) % n]
        };
        let tr = tbl[n - 1][d % n] - tbl[(a + n - 1) % n][d % n];
        let bl = tbl[c % n][n - 1] - tbl[c % n][(b + n - 1) % n];
        let br = tbl[c % n][d % n];
        let t = if a % n == 0 {
            tbl[n - 1][n - 1]
        } else {
            tbl[n - 1][n - 1] - tbl[(a + n - 1) % n][n - 1]
        };
        let b = tbl[c % n][n - 1];
        let l = if b % n == 0 {
            tbl[n - 1][n - 1]
        } else {
            tbl[n - 1][n - 1] - tbl[n - 1][(b + n - 1) % n]
        };
        let r = tbl[n - 1][d % n];

        println!("{}, {}, {}, {}", tl, tr, bl, br);
        println!("{}, {}, {}, {}", t, l, r, b);

        let iw = ((d % n) - (b % n)) / n;
        let ih = ((c % n) - (a % n)) / n;
        println!("(iw, ih) = ({}, {})", iw, ih);
    }
}

/*
3 3
WWB
BBW
WBW
1 2 3 4
0 3 4 5
1 3 4 5

 */
