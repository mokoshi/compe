use proconio::input;

fn main() {
    input! {
        a: [[usize; 9]; 9]
    }
    for i in 0..9 {
        let mut flag = vec![false; 10];
        for j in 0..9 {
            if flag[a[i][j]] == true {
                println!("No");
                return;
            }
            flag[a[i][j]] = true;
        }
    }
    for i in 0..9 {
        let mut flag = vec![false; 10];
        for j in 0..9 {
            if flag[a[j][i]] == true {
                println!("No");
                return;
            }
            flag[a[j][i]] = true;
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            let mut flag = vec![false; 10];
            for ii in 0..3 {
                for jj in 0..3 {
                    let i = i * 3 + ii;
                    let j = j * 3 + jj;
                    if flag[a[i][j]] == true {
                        println!("No");
                        return;
                    }
                    flag[a[i][j]] = true;
                }
            }
        }
    }
    println!("Yes");
}
