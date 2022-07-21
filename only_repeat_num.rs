fn repeated_number(nums: &mut [usize]) -> &usize {
    for i in 0..nums.len() {
        while nums[i] != i+1 {
            if nums[nums[i]-1] == nums[i] {
                return &nums[i];
            } else {
                nums.swap(nums[i]-1, i);
            }   
        }
    }
    &0
}

fn main() {
    println!("{}", repeated_number(&mut [4, 1, 5, 9, 3, 8, 6, 7, 2, 3]));
}