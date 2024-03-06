use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [String; n],
    }
    let mut result_arr: Vec<String> = vec![];
    for (index, v) in a.iter().enumerate() {
        if index == 0 {
            let first_string = a[index + 1].clone();
            let first_char = first_string[0..1].to_string().clone();
            let item = first_char + &v[0..v.len() - 1];
            result_arr.push(item);
        } else if index == a.len() - 1 {
            let last_string = a[index - 1].clone();
            let last_char = last_string[last_string.len() - 1..].to_string().clone();
            let item = v[1..v.len()].to_string() + &last_char;
            result_arr.push(item);
        } else {
            let first_string = a[index + 1].clone();
            let first_char = first_string[0..1].to_string().clone();
            let last_string = a[index - 1].clone();
            let last_char = last_string[last_string.len() - 1..].to_string().clone();
            let item = first_char + &v[1..v.len() - 1] + &last_char;
            result_arr.push(item)
        }
    }
    for val in result_arr {
        println!("{}", val);
    }
}
