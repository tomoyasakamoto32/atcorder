use proconio::input;

fn main() {
    input! {
        w: i32,
        b: i32,
    }
    let mut t = "wbwbwwbwbwbw".to_string();
    for _ in 0..100 {
        t += "wbwbwwbwbwbw"
    }
    for i in 0..12 {
        let mut nw = 0;
        let mut nb = 0;

        for j in 0..w + b {
            let index = i + j;
            let a = &t[(index - 1) as usize..index as usize];
            if a == "w" {
                nw += 1
            } else {
                nb += 1
            }
        }
        if nw == w && nb == b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
