fn sum_of_k_number(v: &mut Vec<i32>, sum: i32, n: i32) {
    let mut display = String::new();
    if  n <= 0 || sum <= 0 {
        return;
    } else if sum == n {
        v.iter().fold(&mut display, |acc, i| {
            acc.push_str(&format!("{}+",i));
            acc
        });
        println!("{}{}",display, n);
    }
    v.push(n);
    sum_of_k_number(v, sum-n, n-1);
    v.pop();
    sum_of_k_number(v, sum, n-1);
}
fn main(){
    let mut v1: Vec<i32> = Vec::new();
    sum_of_k_number(&mut v1, 13, 8)
}
