use proconio::input;

fn main() {
    input! {
        mut s: String,
        t: String,
    }

    let mut flag = true;
    for (index, t_item) in t.chars().enumerate() {
        let ind = s.chars().position(|r| r == t_item.to_ascii_lowercase());
        if index + 1 == 3 && t_item == 'X' {
            break;
        }
        if let Some(x) = ind {
            let current = &s[x + 1..].to_string();
            s = current.to_string().clone();
        } else {
            flag = false;
            break;
        }
    }

    if flag == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
