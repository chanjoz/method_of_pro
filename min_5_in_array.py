def five_small(list1):
    five = list1[:5]
    rest = list1[5:]

    for i in rest:
        m = max(five)
        if i < m: 
            five.remove(m)
            five.append(i)

    return five

if __name__ == '__main__':
    a = [112, 32, 344 ,45 , 7, 22, 78, 40, 56, 11]
    print(five_small(a))