def rgb_sort(list_color):
    start = 0
    current = 0
    end = 2
    while current <= end:
        if list_color[current] == 'R':
            list_color[start], list_color[current] = list_color[current], list_color[start]
            start += 1
            current += 1
        elif list_color[current] == 'G':
            current += 1
        else:
            list_color[current], list_color[end] = list_color[end], list_color[current]
            end -= 1
    return list_color

if __name__ == "__main__":
    l1 = ['B', 'R', 'G']
    print(rgb_sort(l1))
