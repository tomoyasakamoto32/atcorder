use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
    }

    let mut scores: HashMap<i32, i32> = a
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, t)| (t, (i as i32)))
        .collect();

    for a_item in a {
        let b = scores.get(&a_item);
        let c_key = a[(a_item - 1) as usize];
        let c = scores.get(&c_key);
        if let Some(x) = b {
            if let Some(y) = c {
                scores.insert(c_key, *x);
                scores.insert(a_item, *y);
            }
        }
    }
}
