fn str_to_num(s: &str) -> i16 {
    let num = s.chars().map(|c| {
        (c as u8) - ('0' as u8)
    }).inspect(|x|println!("beam {}",x))
    .fold(0,|mut acc, c| {
        if acc < i16::MAX/10 {
            acc = acc*10 + c as i16;
        } else {
            acc = i16::MAX;
        }
        acc
    });
    num
}

fn main() {
    let s = "334";
    println!("{}", str_to_num(s));
}