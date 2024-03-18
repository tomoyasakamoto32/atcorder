use proconio::input;

fn main() {
    input! {
        x: i128,
    }
    if x / 10 > 0 {
        if x % 10 != 0 {
            println!("{}", (x / 10) + 1);
        } else {
            println!("{}", x / 10);
        }
    }
    if x / 10 < 0 {
        let x = (x / 10) as f64;
        println!("{}", x.floor());
    }
}
