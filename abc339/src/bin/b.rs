use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        n: i32,
    }
    let mut initial: Vec<Vec<String>> = vec![];
    for _r in 0..h {
        let mut row: Vec<String> = vec![];
        for _i in 0..w {
            row.push(".".to_string());
        }
        initial.push(row);
    }
    let mut current: (usize, usize) = (1, 1);

    for _i in 0..n {
        let row = initial.get(current.0 - 1).unwrap();
        let target = row.get(current.1 - 1).unwrap();
        if target == "." {
            initial[current.0 - 1][current.1 - 1] = "#".to_string();
        } else {
        }
    }
}
