use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [(i32, i32); n]
    }

    for a_item in a.clone() {
        let mut num = 0;
        let mut ind = 0;
        for (usize, b_item) in a.clone().iter().enumerate() {
            let current = (a_item.0 - b_item.0) * (a_item.0 - b_item.0)
                + (a_item.1 - b_item.1) * (a_item.1 - b_item.1);
            if num < current {
                num = current;
                ind = usize;
            }
        }
        println!("{}", ind + 1);
    }
}
