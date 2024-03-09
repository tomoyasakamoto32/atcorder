use proconio::input;

fn main() {
    input! {
        string: String
    }

    let first = string.chars().next().unwrap();
    let count = string.matches(first).count();
    if count >= 2 {
        let mut index = 0;
        for i in string.chars() {
            index += 1;
            if first != i {
                println!("{}", index)
            };
        }
    } else {
        println!("{}", 1);
    }
}
