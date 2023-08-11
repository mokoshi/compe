use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        mut b: [i32; n],
        mut c: [i32; n],
    }
    a.sort();
    b.sort();
    c.sort();

    let mut answer: usize = 0;
    for bb in b {
        let ai = lower_bound_pos(&a, &bb);
        let ci = upper_bound_pos(&c, &bb);
        answer += ai * (n - ci);
    }

    println!("{}", answer);
}

fn upper_bound_pos(list: &Vec<i32>, target: &i32) -> usize {
    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = (left + right) / 2;
        if list[mid] <= *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn lower_bound_pos(list: &Vec<i32>, target: &i32) -> usize {
    let mut left = 0;
    let mut right = list.len();

    while left < right {
        let mid = (left + right) / 2;
        if list[mid] < *target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
