use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    if n % 5 < 3 {
        println!("{}", n - (n % 5))
    } else {
        println!("{}", n + (5 - (n % 5)))
    }
}
