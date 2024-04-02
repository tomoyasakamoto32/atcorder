use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut count = 0;

    for s_item in s.chars() {
        if s_item == 'v' {
            count += 1;
        }
        if s_item == 'w' {
            count += 2;
        }
    }

    println!("{}", count);
}
