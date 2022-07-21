use min_max_heap::MinMaxHeap;

fn five_small(array: &[i32]) -> Vec<i32> {
    let mut five= MinMaxHeap::<i32>::with_capacity(5);
    for i in &array[0..5] {
        five.push(*i);
    }
    let rest = &array[5..];

    rest.iter().fold(&mut five, |acc, x| {
        let m = acc.peek_max().unwrap();
        if x < m {
            acc.replace_max(*x);
            return acc;
        } else {
            acc
        }
    });
    five.into_iter().collect::<Vec<_>>()
}

fn main() {
    let a = &[112, 32, 344 ,45 , 7, 22, 78, 40, 56, 11];
    println!("{:?}", five_small(a));
}