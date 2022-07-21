use std::cmp::max;

fn find_most_repeat_alphabet(s: &str) -> usize {
    let l = s.len();
    if l == 1 {
        return 1;
    } else {
        let r = find_most_repeat_alphabet(&s[1..]);
        let t = &s[1..].chars().fold(1, |mut acc, c|{
            if let Some(first ) = s.chars().nth(0) {
                if c == first {
                    acc += 1;
                }
            } 
            acc
        });
        return max(r, *t);
    }
}

fn main() {
    let s = "";
    println!("{}", find_most_repeat_alphabet(s));
}

