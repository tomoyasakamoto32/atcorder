use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: i32,
        q: i32,
        t: [i32; q],
    }

    let mut scores: HashMap<i32, i32> = HashMap::new();
    for t_item in t {
        let a = scores.get(&t_item);
        if let Some(_) = a {
            scores.remove(&t_item);
        } else {
            scores.insert(t_item, t_item);
        }
    }

    print!("{}", n - scores.len() as i32);
}
