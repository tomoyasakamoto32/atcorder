use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let left_number = a * 2;
    let right_number = a * 2 + 1;

    if left_number == b || right_number == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
