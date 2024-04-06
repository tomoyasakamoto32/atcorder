use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i32,
        ac: [(i32, i32); n]
    }

    let mut scores: HashMap<i32, i32> = HashMap::new();

    for (a, c) in ac {
        let b = scores.get(&c);
        if let Some(v) = b {
            if v > &a {
                scores.insert(c, a);
            }
        } else {
            scores.insert(c, a);
        }
    }

    let x = scores.into_iter().max_by_key(|x| x.1).unwrap();
    println!("{}", x.1)
}
