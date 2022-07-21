// function for reverse part of the string
fn part_str_rev(input: &mut String, start: usize, end: usize) {
    *input = input[..start].chars()
                            .chain(input[start..end].chars().rev())
                            .chain(input[end..].chars()).collect(); 
}

fn main() {
    let mut s = "*Ju*lyA*ugus*t".to_string();
    let v: Vec<_> = s.chars().enumerate().filter_map(|t|{
        if t.1 == '*' { Some(t.0) } else { None } 
    }).collect();  // Vec<index of *>

    // for t in v.iter().rev().enumerate() {
    //     part_str_rev(&mut s, t.0, t.1+t.0+1);
    //     part_str_rev(&mut s, t.0+1, t.1+t.0+1);
    // }
    v.iter().rev().enumerate().fold(&mut s, |acc, t|{
        part_str_rev(acc, t.0, t.1+t.0+1);
        part_str_rev(acc, t.0+1, t.1+t.0+1);
            acc
        });
    println!("{}",s);
}