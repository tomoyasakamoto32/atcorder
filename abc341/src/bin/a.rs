use proconio::input;

fn main() {
    input! {
        n: i32
    }

    let r = n + n + 1;

    for i in 0..r {
        if i % 2 == 0 {
            print!("1")
        } else {
            print!("0")
        }
    }
}
