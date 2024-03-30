use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String
    }

    let mut a: Vec<String> = vec![];

    for (index, _) in s.chars().enumerate() {
        for i in index..s.len() {
            let target = &s[index..i + 1];
            a.push(target.to_string())
        }
    }
    let a_hash: HashSet<String> = a.into_iter().collect();

    println!("{}", a_hash.len());
}
