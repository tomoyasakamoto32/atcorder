use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: String,
    }

    for i in 1..n {
        let mut count = 0;
        for u in 0..s.len() {
            if u + i > s.len() || u + i + 1 > s.len() {
                break;
            }
            let left_str = &s[u..u + 1];
            let right_str = &s[u + i..u + i + 1];
            if left_str == right_str {
                break;
            }
            count += 1;
        }
        println!("{}", count);
    }
}
