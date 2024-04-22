use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let num = &s[3..];

    let b: i32 = num.parse().unwrap();

    if b != 316 && b < 350 && b > 0 {
        print!("Yes");
    } else {
        print!("No");
    }
}
