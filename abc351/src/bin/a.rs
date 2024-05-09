use proconio::input;

fn main() {
    input! {
        a: [i32; 9],
        b: [i32; 8],
    }

    let mut total_a = 0;

    for a_item in a {
        total_a += a_item;
    }

    let mut total_b = 0;

    for b_item in b {
        total_b += b_item;
    }

    println!("{}", total_a - total_b + 1);
}
