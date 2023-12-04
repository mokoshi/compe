use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        s: [String; n]
    }
    let mut ans = vec![];
    'root: for i in 0..n {
        if s[i] == t {
            ans.push(i);
            continue;
        }
        if s[i].len() == t.len() {
            let mut diff = 0;
            for j in 0..s[i].len() {
                if s[i][j..j + 1] != t[j..j + 1] {
                    if diff > 0 {
                        continue 'root;
                    }
                    diff += 1;
                }
            }
            ans.push(i);
            continue;
        }

        let (a, b) = if s[i].len() > t.len() {
            (&t[..], &s[i][..])
        } else {
            (&s[i][..], &t[..])
        };
        if b.len() - a.len() != 1 {
            continue;
        }
        let mut diff = 0;
        for j in 0..b.len() {
            if a[j - diff..j + 1 - diff] != b[j..j + 1] {
                if diff > 0 {
                    continue 'root;
                }
                diff += 1;
            } else {
                if diff == 0 && j == a.len() - 1 {
                    ans.push(i);
                    continue 'root;
                }
            }
        }
        ans.push(i);
    }
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
}

/*
8 aa
a
b
aa
ab
ba
abc
bac
cba

6 abcde
abcde
fbcde
afcde
abfde
abcfe
abcdf

5 abcde
bcde
acde
abde
abce
abcd

6 abcde
fabcde
afbcde
abfcde
abcfde
abcdfe
abcdef

2 abcde
abcdefg
abcdeg

6 aaabbb
baabbb
ababbb
aabbbb
aaaabb
aaabab
aaabba

5 aaabbb
aaabbb
aaaabb
aaaaab
aaaaaab
aaaaaaa


 */
