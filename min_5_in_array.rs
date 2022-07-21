fn five_small(array: &[i32]) -> Vec<i32> {
    let mut five= Vec::from(&array[0..5]);
    let rest = &array[5..];

    rest.iter().fold(&mut five, |acc, x| {
        let m = acc.iter().max().unwrap();
        if x < m {
            let mindex = acc.iter().position(|v|*v == *m).unwrap();
            acc.remove(mindex);
            acc.push(*x);
            return acc;
        } else {
            acc
        }
    });
    five
}

fn main() {
    let a = &[112, 32, 344 ,45 , 7, 22, 78, 40, 56, 11];
    println!("{:?}", five_small(a));
}