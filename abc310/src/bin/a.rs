use proconio::input;

fn main() {
    input! {
        first: (i32, i32, i32),
        second: [i32; first.0],
    }

    let min = second.iter().min().unwrap();

    if first.1 > min + first.2 {
        println!("{}", min + first.2);
    } else {
        println!("{}", first.1);
    }
}
