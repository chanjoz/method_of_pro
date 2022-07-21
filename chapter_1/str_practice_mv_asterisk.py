def part_rev(start, end):
    global in_str
    in_str = in_str[:start]+''.join(reversed(in_str[start:end]))+in_str[end:]

if __name__ == '__main__':
    in_str = "*Ju*lyA*ugus*t"
    list1 = []
    for i, c in enumerate(in_str):
        if c == '*':
            list1.append(i)

    for t0, t1 in enumerate(reversed(list1)):
        part_rev(t0, t0+t1+1)
        part_rev(t0+1, t0+t1+1)

    print(in_str)