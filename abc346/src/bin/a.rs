use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut arr: Vec<String> = vec![];
    for i in 0..n - 1 {
        let a = a[i] * a[i + 1];
        arr.push(a.to_string());
    }
    print!("{}", arr.join(" "));
}
