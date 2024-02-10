use proconio::input;

fn main() {
    input! {
        query_count: i32,
        query: [(i32, i32); query_count],
    }

    let mut list: Vec<i32> = vec![];

    for q in query {
        if q.0 == 1 {
            list.push(q.1);
        }
        if q.0 == 2 {
            let index = list.len() - q.1 as usize;
            println!("{:?}", &list.get(index).unwrap())
        }
    }
}
