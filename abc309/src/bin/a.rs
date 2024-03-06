use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let diff = b - a;
    if a % 3 == 0 {
        println!("No");
    } else {
        if diff == 1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
