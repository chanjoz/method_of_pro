use std::collections::HashMap;
use std::io;

fn main() {
    let mut input_string = String::new();
    let mut input_char_hash= HashMap::<char, usize>::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line!");
    for c in input_string.chars() {
        *input_char_hash.entry(c).or_insert(0) += 1;
    }
    println!("{:?}", input_char_hash);
}