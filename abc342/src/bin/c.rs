use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _: i32,
        string: String,
        q: i32,
        c: [(char, char); q],
    }
    let mut map: HashMap<char, char> = HashMap::new();
    for i in String::from_utf8((b'a'..b'z').collect()).unwrap().split("") {
        if i != "" {
            map.insert(i.chars().next().unwrap(), i.chars().next().unwrap());
        }
    }
    for (c, d) in c {
        for (k, v) in map.clone() {
            if c == v {
                map.insert(k, d);
            }
        }
    }
    let mut result: String = "".to_string();
    for s in string.chars() {
        let val = map.get(&s);
        if let Some(v) = val {
            result.push(v.clone());
        }
    }
    println!("{}", result);
}
