use proconio::input;

fn main() {
    input! {
        n: u128,
        a: u128,
        b: u128,
        d: [u128; n]
    }

    let mut flag = false;
    for d_item in d {
        let one_week = a + b;
        if d_item % one_week > a {
            flag = true;
        }
    }

    if flag == true {
        println!("No");
    } else {
        println!("Yes");
    }
}
