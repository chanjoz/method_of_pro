def not_repeat_num(nums):
    sum = 0
    sum_1 = 0
    for i in nums:
        sum ^= i
    for i in nums:
        x = sum & -sum
        if x == x & i:   
            sum_1 ^= i 
    return(sum_1, sum^sum_1)
    

if __name__ == "__main__":
    print(not_repeat_num([3, 4, 7, 7, 11, 4, 5, 3, 20, 5]))