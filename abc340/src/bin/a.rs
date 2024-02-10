use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        d: i32,
    }

    let mut k = 0;
    loop {
        if a + k > b {
            break;
        };
        if k == 0 {
            print!("{}", a + k);
        } else {
            print!(" {}", a + k);
        }

        k += d;
    }
}
