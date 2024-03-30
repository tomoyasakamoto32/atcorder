use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        a: [i32; n]
    }

    let mut arr: Vec<String> = vec![];
    for a_item in a {
        if a_item % k == 0 {
            let b = a_item / k;
            arr.push(b.to_string())
        }
    }

    print!("{}", arr.join(" "));
}
