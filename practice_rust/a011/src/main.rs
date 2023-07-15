use proconio::input;

// https://atcoder.jp/contests/abc128/tasks/abc128_c
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut km = vec![0; m];
    let mut sm = vec![vec![]; m];

    for j in 0..m {
        input!(k: usize, s: [usize; k]);
        km[j] = k;
        sm[j] = s;
    }
    input!(p: [usize; m]);

    let mut answer = 0;
    'root: for i in 0..1 << n {
        for j in 0..m {
            let mut on = 0;
            for x in 0..km[j] {
                if i & 1 << (sm[j][x] - 1) != 0 {
                    on += 1;
                }
            }
            if on % 2 != p[j] {
                continue 'root;
            }
        }
        answer += 1;
    }

    println!("{}", answer);
}
