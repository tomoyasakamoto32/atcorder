use proconio::input;

fn main() {
    input!(
        height: i32,
        _width: i32,
        strings: [String; height],
    );

    for string in strings {
        for _i in 0..2 {
            println!("{}", string)
        }
    }
}
