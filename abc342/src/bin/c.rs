use std::io::{self, BufRead};

fn main() {
    // 標準入力からの値の読み込み
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // nの読み込み
    let _n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // pの読み込み
    let mut p: String = lines.next().unwrap().unwrap().trim().to_string();
    
    // qの読み込み
    let q: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // queryの読み込み
    let mut query = Vec::new();
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        let s1 = parts[0].to_string();
        let s2 = parts[1].to_string();
        query.push((s1, s2));
    }

    // クエリの実行
    for (s1, s2) in query {
        p = p.replace(&s1, &s2);
    }

    // 結果の表示
    println!("{}", p);
}

