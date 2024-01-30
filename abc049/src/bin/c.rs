use proconio::input;

fn main() {
    input! {
        mut s: String
    }
    let search_strings = [
        "dreameraser",
        "dreamerase",
        "eraser",
        "erase",
        "dreamer",
        "dream",
    ];
    for search_string in search_strings {
        s = s.replace(search_string, "");
    }

    if s.len() > 0 {
        println!("NO");
    } else {
        println!("YES");
    }
}

// use regex::Regex;

// fn main() {
//     input! {
//         s: String,
//     }
//     let mut res = s;
//     for t in &["eraser", "erase", "dreamer", "dream"] {
//         let re = Regex::new(t).unwrap();
//         res = re.replace_all(&res, "").to_string();
//     }
//     if res.is_empty() {
//         println!("YES")
//     } else {
//         println!("NO")
//     }
// }
