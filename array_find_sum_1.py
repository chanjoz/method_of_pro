def check_two_sum(list1, expect_sum):
    start = 0
    end = len(list1)-1
    while start < end:
        sum = list1[start]+list1[end]
        if sum == expect_sum:
            return True
        elif sum > expect_sum:
            end -= 1
        else: 
            start += 1
    return False

if __name__ == '__main__':
    l1 = [1, 2, 5, 7, 14, 22]
    print(check_two_sum(l1, 20))
