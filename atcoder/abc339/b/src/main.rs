use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize
    }
    let mut b = vec![vec![0; w]; h];
    let mut mv = (-1, 0);
    let mut pos = (0, 0);
    for _ in 0..n {
        if b[pos.0][pos.1] == 0 {
            b[pos.0][pos.1] = 1;
            mv = (mv.1, -mv.0);
        } else {
            b[pos.0][pos.1] = 0;
            mv = (-mv.1, mv.0);
        }
        pos.0 = (pos.0 as i32 + h as i32 + mv.0) as usize % h;
        pos.1 = (pos.1 as i32 + w as i32 + mv.1) as usize % w;
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", if b[i][j] == 0 { '.' } else { '#' });
        }
        println!();
    }
}
