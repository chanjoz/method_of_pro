def is_palindrome(istr):
    l = len(istr)
    for i in range(0, l//2):
        if istr[i] != istr[l-(i+1)]:
            return False
        else:
            continue
    return True

if __name__ == "__main__":
    print(is_palindrome('userresu'))