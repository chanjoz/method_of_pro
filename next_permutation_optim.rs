fn calc_all_permutation(v: &mut [u8]) -> bool {
    let num = v.len();
    let op = v.windows(2).rposition(|w|w[0] < w[1]);
    let mut index = 0 as usize;
    match op {
        Some(i) => {
            index = i;
        },
        None => {
            return false;
        }
    }

    let op = v.iter().rposition(|u|u > &v[index]);
    let mut k = 0 as usize;
    if let Some(i) = op {
        k = i;
    }

    v.swap(index, k);
    v[index+1..num].reverse();   

    return true;
}

fn main() {
    let s = "12345";
    let mut v = s.chars().map(|c| c as u8). collect::<Vec<u8>>();
    loop {
        if calc_all_permutation(&mut v) {
            let ns = v.iter().map(|x|*x as char).collect::<String>();
            println!("beam {:?}", ns);

        } else {
            break;
        }
    }
}