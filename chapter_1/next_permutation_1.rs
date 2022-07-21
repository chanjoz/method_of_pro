fn calc_all_permutation(s: &str) -> bool {
    let mut v: Vec<u8> = s.chars().map(|c| c as u8)
                .collect();
    let num = s.len();
    let mut i = num;
    let mut b = false;
    while i >= 2 {
        if &v[i-2] >= &v[i-1] {
            i -= 1;
        } else {
            b= true;
            break;
        }
    }
    if b == false {
        return false;
    }
      
    let mut k = num -1;
    loop {
        if &v[k] < &v[i-2] {
            k -= 1;
        } else {
            break;
        }
    }

    v.swap(i-2, k);
    v[i-1..num].reverse();
    let s = v.into_iter().map(|x|x as char)
        .collect::<String>();
    println!("{}", s);    

    return b;
}

fn main() {
    let s = "54312";
    println!("{}", calc_all_permutation(s));
}