import heapq

def five_small(list1):
    five = [-i for i in list1[:5]]
    rest = [-i for i in list1[5:]]

    heapq.heapify(five)
     
    for i in rest:
        heapq.heappushpop(five, i)
    return five

if __name__ == '__main__':
    a = [112, 32, 344 ,45 , 7, 22, 78, 40, 56, 11]
    f = [-i for i in five_small(a)]
    print(f)
    