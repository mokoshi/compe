use proconio::input;

fn main() {
    input! {
        k: i64
    }
    let mut t = vec![-1 as i32; 11];
    for _ in 0..=k {
        for j in 0..10 {
            if (t[j + 1] == -1 || t[j] + 1 < t[j + 1]) && t[j] < 9 {
                if t[j] == -1 {
                    t[j] = j as i32;
                } else {
                    t[j] += 1;
                }
                for l in 0..j {
                    t[l] = l as i32;
                }
                break;
            }
        }
    }
    for i in 0..11 {
        if t[10 - i] != -1 {
            print!("{}", t[10 - i]);
        }
    }
    println!("");
    /*
    1, 2, 3, 4, 5, 6, 7, 8, 9
    10, 20, 21, 30, 31, 32, ...  -> 10, 12, 15, 19, ...
    210, 310, 320, 321, 410, 420, 421, 430, 432, 431, -> 1, 2+1, 3+2+1, 4+3+2+1, ...
    3210, 4
     */
    // let mut t = vec![vec![0; 10]; 10];
    // let mut c = vec![vec![0; 10]; 10];
    // for i in 0..10 {
    //     t[0][i] = i;
    //     c[0][i] = 1;
    // }
    // t[0][0] = 0;
    // let mut prev = 9;
    // for i in 1..10 {
    //     for j in i..10 {
    //         c[i][j] = (0..j).map(|x| c[i - 1][x]).sum::<usize>();
    //         t[i][j] = prev + c[i][j];
    //         prev = t[i][j];
    //     }
    // }
    // println!("{:?}, {:?}", t, c);
}
