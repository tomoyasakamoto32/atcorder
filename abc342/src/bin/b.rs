use proconio::input;

fn main() {
    input!{
        n: i32,
        p: [i32; n],
        q: i32,
        query: [(i32, i32); q],
    };

    for (q1, q2) in query {
        let mut q1index: i32;
        let mut q2index: i32;
        let mut index = 0; 
        for val in p {
            if q1 == val {
                q1index = index;
            }
            if q2 == val {
                q2index = index;
            }
            index += 1
        }
        if q1index > q2index {
            println!("{}", q1);
        } else {
            println!("{}", q2);
        }
    }
}
