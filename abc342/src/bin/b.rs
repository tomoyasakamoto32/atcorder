use proconio::input;

fn main() {
    input! {
        n: i32,
        p: [i32; n],
        q: i32,
        query: [(i32, i32); q],
    };

    for (q1, q2) in query.into_iter() {
        let q1index = p.iter().position(|&item| item == q1).unwrap();
        let q2index = p.iter().position(|&item| item == q2).unwrap();

        if q1index > q2index {
            println!("{}", q2);
        } else {
            println!("{}", q1);
        }
    }
}
