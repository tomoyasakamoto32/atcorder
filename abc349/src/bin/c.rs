use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let mut scores: HashMap<char, i32> = HashMap::new();
    for t in s.chars() {
        let a = scores.get(&t);
        if let Some(v) = a {
            scores.insert(t, *v + 1);
        } else {
            scores.insert(t, 1);
        }
    }

    let mut flag = true;
    for (index, t_item) in t.chars().enumerate() {
        if index + 1 == 3 && t_item == 'X' {
            break;
        }
        let c = t_item.clone().to_ascii_lowercase();
        let q = scores.get(&c);
        if let Some(v) = q {
            if v.clone() <= 0 {
                flag = false;
            }
            scores.insert(c, *v - 1);
        } else {
            flag = false;
        }
    }

    if flag == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
