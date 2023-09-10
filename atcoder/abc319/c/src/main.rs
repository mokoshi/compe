use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        cc: [[i32; 3]; 3]
    }

    let mut gakkari = 0;
    for p in (0..9).permutations(9) {
        let mut rows = vec![vec![]; 3];
        let mut cols = vec![vec![]; 3];
        let mut rd = vec![];
        let mut ru = vec![];

        for pi in &p {
            let r = pi / 3;
            let c = pi % 3;
            let v = cc[r][c];

            rows[r].push(v);
            cols[c].push(v);
            if (rows[r].len() == 2 && rows[r][0] == rows[r][1])
                || (cols[c].len() == 2 && cols[c][0] == cols[c][1])
            {
                gakkari += 1;
                break;
            }

            if r == c {
                rd.push(v);
                if rd.len() == 2 && rd[0] == rd[1] {
                    gakkari += 1;
                    break;
                }
            }
            if r + c == 2 {
                ru.push(v);
                if ru.len() == 2 && ru[0] == ru[1] {
                    gakkari += 1;
                    break;
                }
            }
        }
    }

    println!("{}", 1.0 - (gakkari as f64) / 362880.0)
}
