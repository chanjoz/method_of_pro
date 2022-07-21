def str_num(str):
    num = 0
    for c in str: 
        t = ord(c) - ord('0')
        num = 10*num+t
    print(f'beam {int(str,10)}')
    return num

if __name__ == '__main__':
    print(str_num("344"))