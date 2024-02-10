use proconio::input;

fn main() {
    input! {
        N: u128
    }
    let above = (N / 2) + 1;
    let below = (N / 2);
    println!("{}, {}", above, below);
}
