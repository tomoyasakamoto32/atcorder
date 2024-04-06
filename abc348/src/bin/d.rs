use proconio::input;

fn main() {
    input! {
        h: i32,
        w: i32,
        a: [String; h],
        n: i32,
        r: [(i32, i32, i32); n],
    }

    let mut start_point: (i32, i32) = (0, 0);
    let mut end_point: (i32, i32) = (0, 0);
    for (h_index, a_item) in a.iter().enumerate() {
        for (w_index, a_item_item) in a_item.chars().enumerate() {
            if a_item_item == 'S' {
                let h_index_ = h_index + 1;
                let w_index_ = w_index + 1;
                start_point = (h_index_ as i32, w_index_ as i32);
            }
            if a_item_item == 'T' {
                let h_index_ = h_index + 1;
                let w_index_ = w_index + 1;
                end_point = (h_index_ as i32, w_index_ as i32);
            }
        }
    }

    let mut has_initial_energy = false;
    for r_item in r {
        if r_item.0 == start_point.0 && r_item.1 == start_point.1 {
            has_initial_energy = true
        }
    }
    if has_initial_energy == false {
        println!("No");
        return;
    }
}
