use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let s = 3_usize.pow(n as u32);
    let mut t = vec![vec!['.'; s]; s];

    if n == 0 {
        println!("#");
        return;
    }
    t[0][0] = '#';
    for i in 1..=n {
        let ps = 3_usize.pow((i - 1) as u32);
        for x in 0..ps * 3 {
            for y in 0..ps * 3 {
                if x < ps && y < ps {
                    continue;
                }
                if x >= ps && x < ps * 2 && y >= ps && y < ps * 2 {
                    continue;
                }
                t[y][x] = t[y % ps][x % ps];
            }
        }
    }

    for y in 0..s {
        println!("{}", t[y].iter().join(""));
    }
}
