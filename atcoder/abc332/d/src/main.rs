use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
        b: [[u64; w]; h],
    }
    let mut ans = i32::MAX;
    for wp in (0..w).permutations(w) {
        // println!("{:?}, {}", wp, calc(&wp));
        'l: for hp in (0..h).permutations(h) {
            for x in 0..w {
                for y in 0..h {
                    if a[hp[y]][wp[x]] != b[y][x] {
                        continue 'l;
                    }
                }
            }
            ans = ans.min(calc(&wp) + calc(&hp));
        }
    }
    println!("{}", if ans == i32::MAX { -1 } else { ans })
}

fn calc(p: &Vec<usize>) -> i32 {
    let len = p.len();
    let mut a = p.clone();
    let mut c = 0;
    for i in 0..len - 1 {
        for j in 0..len - 1 - i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                c += 1;
            }
        }
    }
    c
}
