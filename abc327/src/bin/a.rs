use proconio::input;

fn main() {
    input! {
        _n: i32,
        string: String,
    }
    if string.contains("ab") || string.contains("ba") {
        println!("Yes");
    } else {
        println!("No");
    }
}
