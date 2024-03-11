use proconio::input;
use std::collections::HashSet;

// TODO: a,b,cが0以上100以下のため全て回しても10**6程度、そのため全通りの値を格納するMapを用意してその中にxをまわした要素が含まれるかを検証する方針に修正したい
fn main() {
    input! {
        an: i32,
        a: [i32; an],
        bn: i32,
        b: [i32; bn],
        cn: i32,
        c: [i32; cn],
        xn: i32,
        x: [i32; xn],
    }

    let a_set: HashSet<_> = a.iter().cloned().collect();
    let b_set: HashSet<_> = b.iter().cloned().collect();
    let c_set: HashSet<_> = c.iter().cloned().collect();

    for i in x {
        let mut flag = false;
        for aitem in a_set.iter() {
            for bitem in b_set.iter() {
                if c_set.contains(&(i - aitem - bitem)) {
                    flag = true;
                    break;
                }
            }
            if flag {
                break;
            }
        }
        if flag {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
