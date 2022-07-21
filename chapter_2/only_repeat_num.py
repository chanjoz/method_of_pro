def repeated_number(nums):
    for i in range(0, len(nums)):
        while nums[i] != i+1:
            if nums[nums[i]-1] == nums[i]:
                return nums[i]
            else:
                nums[nums[i]-1], nums[i] = nums[i], nums[nums[i]-1]

if __name__ == "__main__":
    print(repeated_number([4, 1, 5, 9, 2, 8, 6, 7, 2, 3]))