use proconio::input;

fn main() {
    input! {
        string: String
    };

    let a: Vec<&str> = string.split('.').collect();
    match a.last() {
        Some(s) => println!("{}", s),
        None => panic!(),
    }
}
