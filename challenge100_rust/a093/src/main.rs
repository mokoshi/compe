use proconio::{input, marker::Chars};

type Board = Vec<Vec<u32>>;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h]
    };

    let mut b: Board = vec![vec![]; h];
    for i in 0..h {
        b[i] = c[i].iter().map(|&c| c.to_digit(10).unwrap()).collect();
    }

    let mut ans = 0;
    for y in 0..h {
        for x in 0..w {
            let mut b: Board = vec![vec![]; h];
            for i in 0..h {
                b[i] = c[i].iter().map(|&c| c.to_digit(10).unwrap()).collect();
            }
            b[y][x] = 0;

            let mut points = 0;
            let mut i = 0;
            loop {
                do_drop(&mut b, h, w);
                let p = do_remove(&mut b, i, h, w, k);
                if p == 0 {
                    break;
                }
                points += p;
                i += 1;
            }
            ans = ans.max(points);
        }
    }
    println!("{}", ans);
}

fn print_board(b: &Board, h: usize, w: usize) {
    println!("-- Board --");
    for y in 0..h {
        for x in 0..w {
            print!("{}", b[y][x]);
        }
        println!()
    }
    println!("-----------");
}

fn do_drop(b: &mut Board, h: usize, w: usize) {
    for x in 0..w {
        let mut bs = vec![];
        for y in 0..h {
            if b[y][x] != 0 {
                bs.push(b[y][x]);
            }
        }
        for y in 0..h {
            b[h - 1 - y][x] = if y < bs.len() {
                bs[bs.len() - 1 - y]
            } else {
                0
            }
        }
    }
}

fn do_remove(b: &mut Board, i: u32, h: usize, w: usize, k: usize) -> i32 {
    let mut total = 0;
    for y in 0..h {
        let mut current = b[y][0];
        let mut cnt = 1;
        let mut start = 0;
        for x in 1..w + 1 {
            let c = if x == w { 0 } else { b[y][x] };
            if current == c {
                cnt += 1;
            } else {
                if cnt >= k && current != 0 {
                    for i in start..x {
                        b[y][i] = 0;
                    }
                    total += cnt as i32 * current as i32;
                }
                current = c;
                cnt = 1;
                start = x;
            }
        }
    }
    total * 2_i32.pow(i)
}
