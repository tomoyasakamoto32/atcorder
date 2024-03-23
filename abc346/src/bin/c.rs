use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u128,
        k: u128,
        a: [u128; n]
    }

    let a_set: HashSet<u128> = a.into_iter().collect();

    let mut num = 0;

    for i in 1..k + 1 {
        if a_set.contains(&i) == false {
            num += i;
        }
    }

    println!("{}", num);
}
