use proconio::input;

fn main() {
    input! {
        n: i32,
        ts: [(i32, i32, i32); n],
    }

    let mut flag = true;

    for (i, value) in ts.iter().enumerate() {
        if i == 0 {
            if value.1 + value.2 < value.0 {
                flag = false;
                break;
            }
            if value.0 % 2 == 0 {
                if value.1 + value.2 % 2 != 0 {
                    flag = false;
                    break;
                }
            } else {
                if value.1 + value.2 % 2 == 0 {
                    flag = false;
                    break;
                }
            }
            continue;
        };

        let before_distance = ts[i - 1];
        let abs_distance =
            (before_distance.1 - value.1).abs() + (before_distance.2 - value.2).abs();
        let time = before_distance.0 - value.0;

        if abs_distance < time {
            flag = false;
            break;
        }

        if time % 2 == 0 {
            if abs_distance % 2 != 0 {
                flag = false;
                break;
            }
        } else {
            if abs_distance % 2 == 0 {
                flag = false;
                break;
            }
        }
    }

    if flag == false {
        println!("No");
    } else {
        println!("Yes");
    }
}
