use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n - 1],
    }

    let mut total: i32 = 0;

    for a_item in a {
        total += a_item
    }

    if total <= 0 {
        println!("{}", total.abs());
    } else {
        println!("-{}", total);
    }
}
