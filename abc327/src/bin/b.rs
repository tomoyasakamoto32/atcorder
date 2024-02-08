use proconio::input;

fn main() {
    input! {
        n: i128,
    }

    if n == 1 {
        println!("{}", n);
    }

    for i in 1..n {
        let mut num = i;
        for _ in 1..i {
            num *= i
        }

        if num == n {
            println!("{}", i);
            break;
        };

        if num > n {
            println!("-1");
            break;
        };
    }
}
