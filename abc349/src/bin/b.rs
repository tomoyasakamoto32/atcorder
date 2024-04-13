use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    }

    let mut scores: HashMap<char, i32> = HashMap::new();
    for s_char in s.chars() {
        let a = scores.get(&s_char);
        if let Some(v) = a {
            scores.insert(s_char, *v + 1);
        } else {
            scores.insert(s_char, 1);
        }
    }

    let mut scores2: HashMap<i32, Vec<char>> = HashMap::new();
    for (c, value) in scores {
        let a = scores2.get(&value);
        if let Some(v) = a {
            let mut new_a: Vec<char> = v.clone();
            new_a.push(c);
            scores2.insert(value, new_a);
        } else {
            let mut arr: Vec<char> = vec![];
            arr.push(c);
            scores2.insert(value, arr);
        }
    }

    let mut flag = false;
    for (_, value) in scores2 {
        if value.len() != 2 {
            flag = true;
        }
    }

    if flag == true {
        print!("No");
    } else {
        print!("Yes");
    }
}
