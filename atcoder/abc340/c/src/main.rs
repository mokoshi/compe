use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64
    }
    let mut table = HashMap::new();
    println!("{}", dfs(n, &mut table));
}

fn dfs(p: u64, table: &mut HashMap<u64, u64>) -> u64 {
    if p <= 1 {
        return 0;
    }
    if table.contains_key(&p) {
        return *table.get(&p).unwrap();
    }

    let a = p / 2;
    let b = p - a;
    let cost = p + if a >= 2 { dfs(a, table) } else { 0 } + if b >= 2 { dfs(b, table) } else { 0 };
    table.insert(p, cost);
    return cost;
}
