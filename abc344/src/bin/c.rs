use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        an: i32,
        a: [i32; an],
        bn: i32,
        b: [i32; bn],
        cn: i32,
        c: [i32; cn],
        xn: i32,
        x: [i32; xn],
    }

    let mut add = vec![];
    for a_item in a.iter() {
        for b_item in b.iter() {
            for c_item in c.iter() {
                add.push(a_item + b_item + c_item)
            }
        }
    }
    let set: HashSet<i32> = add.into_iter().collect();

    for i in x {
        if set.contains(&i) == true {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
