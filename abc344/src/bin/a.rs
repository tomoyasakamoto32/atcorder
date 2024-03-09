use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut array: Vec<usize> = vec![];
    for (usize, i) in s.chars().enumerate() {
        if i == '|' {
            array.push(usize);
        }
    }
    let restlt = s[0..array[0]].to_string() + &s[array[1] + 1..];
    if restlt.len() > 0 {
        println!("{}", restlt);
    }
}
