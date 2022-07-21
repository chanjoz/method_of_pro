input_str = "abc efg   hij"
list1 = input_str.split(' ')
list2 = []
for str1 in list1:
    if str1:
        list2.append(''.join(reversed(str1)))
output_str = ' '.join(list2)
print(output_str)
