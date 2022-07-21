def median3(nums, left, right):
    center = (left+right)//2
    if nums[left] > nums[center]:
        nums[left], nums[center] = nums[center], nums[left]
    if nums[left] > nums[right]:
        nums[left], nums[right] = nums[right], nums[left]
    if nums[center] > nums[right]:
        nums[center], nums[right] = nums[right], nums[center]
    nums[center], nums[right-1] = nums[right-1], nums[center]
    return nums[right-1] 

def quick_select(nums, k, left, right):
    pivot = median3(nums, left, right)
    i = left
    j = right-1
    while(i < j):
        while(nums[i] <= pivot and i < j):
            i+= 1
        nums[j] = nums[i]
        while(nums[j] > pivot and i < j):
            j-= 1
        nums[i] = nums[j] 
    
    nums[i] = pivot

    if k < i:
        return quick_select(nums, k, 0, i-1)
    elif k > i:
        return quick_select(nums, k, i+1, right)
    else:
        return nums[i]
    
if __name__ == "__main__":
    nums = [4, 5, 23, 12, 89, 20, 14, 23, 54, 66, 47, 23]
    print(quick_select(nums, 5, 0, 11))