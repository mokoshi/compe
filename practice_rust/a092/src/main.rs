type Board = Vec<Vec<i32>>;

fn main() {
    loop {
        let h = read_line()[0] as usize;
        if h == 0 {
            break;
        }
        let mut b = vec![vec![]; h];
        for i in 0..h {
            let c = read_line();
            b[i] = c;
        }

        let mut ans = 0;
        loop {
            let mut removed = false;
            for i in 0..h {
                let point = process_remove(i, &mut b);
                if point > 0 {
                    ans += point;
                    removed = true;
                }
            }
            // print_board(&b, h);
            if !removed {
                println!("{}", ans);
                break;
            }
            process_fall(&mut b, h);
            // print_board(&b, h);
        }
    }
}

fn print_board(b: &Board, h: usize) {
    println!("--------------");
    for i in 0..h {
        for j in 0..5 {
            print!("{} ", b[i][j]);
        }
        println!("");
    }
    println!("--------------");
}

fn process_remove(h: usize, b: &mut Board) -> i32 {
    let mut current = b[h][0];
    let mut cnt = 1;
    let mut start = 0;
    for i in 1..5 {
        if b[h][i] == current {
            cnt += 1;
        } else {
            current = b[h][i];
            cnt = 1;
            start = i;
        }
        if cnt >= 3 {
            let mut removed = 0;
            for i in start..5 {
                if b[h][i] == current {
                    b[h][i] = 0;
                    removed += 1;
                } else {
                    break;
                }
            }
            return current * removed;
        }
    }
    0
}

fn process_fall(b: &mut Board, h: usize) {
    for i in 0..5 {
        let mut balls = vec![];
        for j in 0..h {
            if b[h - 1 - j][i] != 0 {
                balls.push(b[h - 1 - j][i]);
            }
        }
        for j in 0..h {
            b[h - 1 - j][i] = if j < balls.len() { balls[j] } else { 0 }
        }
    }
}

fn read_line() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
