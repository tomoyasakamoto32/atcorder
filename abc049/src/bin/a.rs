use proconio::input;

fn main() {
    input! {
        c: char
    }
    let vowel = ['a', 'i', 'u', 'e', 'o'];

    let a = vowel.iter().find(|&&x| x == c);
    match a {
        Some(_x) => println!("vowel"),
        None => println!("consonant"),
    }
}
