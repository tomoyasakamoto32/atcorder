use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    for i in 1..n + 1 {
        if i % 3 == 0 {
            print!("{}", 'x');
        } else {
            print!("{}", 'o');
        }
    }
}
