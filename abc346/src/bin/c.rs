use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: u128,
        k: u128,
        a: [u128; n]
    }

    let a_hash: HashSet<u128> = a.into_iter().collect();

    let mut ans = (k.clone() * (k.clone() + 1)) / 2;

    for a_item in a_hash {
        if a_item <= k {
            ans -= a_item;
        }
    }

    println!("{}", ans);
}
