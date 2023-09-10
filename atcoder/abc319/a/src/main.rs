use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: String
    }

    let rate_map = HashMap::from([
        (String::from("tourist"), 3858),
        (String::from("ksun48"), 3679),
        (String::from("Benq"), 3658),
        (String::from("Um_nik"), 3648),
        (String::from("apiad"), 3638),
        (String::from("Stonefeang"), 3630),
        (String::from("ecnerwala"), 3613),
        (String::from("mnbvmar"), 3555),
        (String::from("newbiedmy"), 3516),
        (String::from("semiexp"), 3481),
    ]);
    println!("{}", rate_map.get(&n).unwrap());
}
