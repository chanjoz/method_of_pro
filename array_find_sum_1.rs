fn check_two_sum(v: &[u32], expect_sum: u32) -> bool {
    let mut start = 0 as usize;
    let mut  end = v.len()-1;
    while start < end {
        let sum = v[start]+v[end];
        if sum == expect_sum {
            return true;
        } else if sum > expect_sum {
            end -= 1;
        } else {
            start += 1;
        }
    }
    false
}
fn main() {
    let v1 = vec![1, 2, 5, 7, 14, 22];
    println!("{}", check_two_sum(&v1, 20));
}
