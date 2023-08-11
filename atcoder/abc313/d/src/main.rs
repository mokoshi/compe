use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize
    }

    let mut answer = vec![-1; n];
    answer[0] = 0;
    let mut prev_s = 0;
    let mut first_s = 0;
    for i in 0..n - k + 1 {
        println!(
            "? {} {}",
            i + 1,
            (n - k + 1..n)
                .map(|x| (x % n + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        input! { s: usize }

        if i == 0 {
            first_s = s;
        }

        if first_s == s {
            answer[i] = answer[0];
        } else {
            answer[i] = 1 - answer[0];
        }
        prev_s = s;
    }
    for i in 0..k - 1 {
        println!(
            "? {} 1 {}",
            (n - k..=n - k + i)
                .map(|x| (x % n + 1).to_string())
                .collect::<Vec<_>>()
                .join(" "),
            (n - k + i + 2..n)
                .map(|x| (x % n + 1).to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        input! { s: usize }

        if prev_s == s {
            answer[n - k + i + 1] = answer[0];
        } else {
            answer[n - k + i + 1] = 1 - answer[0];
        }
        prev_s = s;

        if i == k - 2 {
            if answer.iter().sum::<i64>() % 2 != s as i64 {
                for i in 0..n {
                    answer[i] = 1 - answer[i];
                }
            }
        }
    }

    println!(
        "! {}",
        answer
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
