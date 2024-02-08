use std::ops::Add;

use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    let c: i32 = a.add(&b).parse().unwrap();
    let mut flag = false;
    for i in 1..c + 1 {
        if i * i == c {
            flag = true;
        };
        if c / i < i {
            break;
        }
    }

    if flag == true {
        println!("Yes")
    } else {
        println!("No")
    };
}
