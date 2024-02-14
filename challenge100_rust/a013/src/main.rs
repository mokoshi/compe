use proconio::input;

// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e
fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[usize; c]; r],
    }

    let mut answer = 0;
    for i in 0..1 << r {
        let mut b = a.clone();
        for j in 0..r {
            if 1 << j & i != 0 {
                for k in 0..c {
                    b[j][k] = 1 - b[j][k];
                }
            }
        }

        let mut cnt = 0;
        for x in 0..c {
            let mut column = 0;
            for y in 0..r {
                column += b[y][x];
            }
            cnt += column.max(r - column);
        }

        answer = answer.max(cnt);
    }

    println!("{}", answer);
}
