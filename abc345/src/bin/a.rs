use proconio::input;

fn main() {
    input! {
        string: String
    }

    let mut first_flag = false;
    let mut second_flag = true;
    let mut third_flag = false;
    for (index, s) in string.chars().enumerate() {
        if index == 0 && s == '<' {
            first_flag = true;
        }
        if index == string.len() - 1 && s == '>' {
            third_flag = true;
        }
        if index != 0 && index != string.len() - 1 {
            if s != '=' {
                second_flag = false;
            }
        }
    }

    if first_flag && second_flag && third_flag {
        println!("Yes")
    } else {
        println!("No")
    }
}
