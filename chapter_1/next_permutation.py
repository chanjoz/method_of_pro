def calc_permutation():
    global input_str
    list1 = list(input_str)
    flag = False
    l = len(list1)
    for i in range(l-1, 2 ,-1):
        if list1[i-2] < list1[i-1]:
            flag = True
            break
        else:
            continue
    if flag == False:
        return flag


    for j in range(l-1, 0, -1):
        if list1[j] > list1[i-2]:
            break
    list1[i-2], list1[j] = list1[j], list1[i-2]
    list1 = list1[0:i-1] + list1[l:i-2:-1]
    input_str = ''.join(list1)
    return True

if __name__ == '__main__':
    input_str = '54231'
    print(calc_permutation())
    print(input_str)