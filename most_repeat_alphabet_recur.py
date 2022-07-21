def find_most_repeat_alphabet(string1):
    l = len(string1)
    if l == 1:
        return l
    else:
        r = find_most_repeat_alphabet(string1[1:])
        t = 1
        for c in string1[1:]:
            if c == string1[0]:
                t += 1
        return max(r, t)
    
if __name__ == "__main__":
    s = "aaaaabbccc"
    print(find_most_repeat_alphabet(s))