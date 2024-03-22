use proconio::input;

fn main() {
    input! {
        s: [[i32; 9]; 9]
    }
    let first_flag = (1..10).all(|i| s[0].contains(&i));

    let mut new_arr: Vec<i32> = vec![];
    for item in s.clone() {
        let a = item[0].clone();
        new_arr.push(a);
    }
    let second_flag = (1..10).all(|i| new_arr.contains(&i));

    let mut first: Vec<i32> = vec![];
    let mut second: Vec<i32> = vec![];
    let mut third: Vec<i32> = vec![];
    let mut forth: Vec<i32> = vec![];
    let mut fifth: Vec<i32> = vec![];
    let mut six: Vec<i32> = vec![];
    let mut seventh: Vec<i32> = vec![];
    let mut eigth: Vec<i32> = vec![];
    let mut nineth: Vec<i32> = vec![];
    for (i, value) in s.clone().iter().enumerate() {
        if i < 3 {
            for (index, v) in value.clone().iter().enumerate() {
                if index < 3 {
                    first.push(*v);
                }
                if 3 <= index && index < 6 {
                    second.push(*v);
                }
                if 6 <= index && index < 9 {
                    third.push(*v);
                }
            }
        }
        if 3 <= i && i < 6 {
            for (index, v) in value.clone().iter().enumerate() {
                if index < 3 {
                    forth.push(*v);
                }
                if 3 <= index && index < 6 {
                    fifth.push(*v);
                }
                if 6 <= index && index < 9 {
                    six.push(*v);
                }
            }
        }
        if 6 <= i && i < 9 {
            for (index, v) in value.clone().iter().enumerate() {
                if index < 3 {
                    seventh.push(*v);
                }
                if 3 <= index && index < 6 {
                    eigth.push(*v);
                }
                if 6 <= index && index < 9 {
                    nineth.push(*v);
                }
            }
        }
    }

    let mut third_flag = true;
    for item in [
        first, second, third, forth, fifth, six, seventh, eigth, nineth,
    ] {
        if (1..10).all(|v| item.contains(&v)) == false {
            third_flag = false;
        }
    }

    if first_flag && second_flag && third_flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
