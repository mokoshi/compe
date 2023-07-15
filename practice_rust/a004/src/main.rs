use proconio::input;

// https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c
fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [[usize; m]; n]
    };

    let mut answer = 0;
    for i in 0..m {
        for j in i + 1..m {
            let mut point = 0;
            for x in 0..n {
                point += aa[x][i].max(aa[x][j]);
            }
            answer = answer.max(point);
        }
    }
    println!("{}", answer)
}
