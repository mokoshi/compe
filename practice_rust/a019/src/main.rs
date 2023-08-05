use proconio::input;

// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b
fn main() {
    input! {
        d: i32,
        n: i32,
        m: i32,
        mut di: [i32; n-1],
        ki: [i32; m],
    }
    di.sort();
    di.insert(0, 0);
    di.push(d);

    let mut answer = 0;
    for k in ki {
        let mut left: usize = 0;
        let mut right: usize = n as usize;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if k < di[mid] {
                right = mid;
            } else {
                left = mid;
            }
        }

        answer += (k - di[left]).min(di[right] - k);
    }

    println!("{}", answer);
}
