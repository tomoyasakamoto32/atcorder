use std::convert::TryInto;

use proconio::input;

fn main() {
    input! {
        string: String
    }
    let mut result: u64 = 0;
    let alphabet = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    for (str_index, s) in string.chars().enumerate() {
        let l = string.len() - str_index - 1;

        for (alp_index, alp) in alphabet.iter().enumerate() {
            if s == *alp {
                let a: u32 = 26;
                let b: u32 = l.try_into().unwrap();
                let pow = a.pow(b);
                let alp_index: u32 = (alp_index + 1).try_into().unwrap();
                result += u64::from(alp_index) * u64::from(pow);
                break;
            }
        }
    }
    println!("{}", result);
}
