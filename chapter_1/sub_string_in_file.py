import sys

def check_sub_string(source, target):
    word_candi = {}
    for w in target:
        word_candi[w] = word_candi.get(w, 0) + 1
    
    word_window = {}
    match_cnt = 0
    need_cnt = len(word_candi)
    left = 0
    right = 0
    start = 0
    min_len = sys.maxsize

    while right < len(source):
        w1 = source[right]
        if w1 in word_candi:
            word_window[w1] = word_window.get(w1, 0) + 1
            if word_window[w1] == word_candi[w1]:
                match_cnt += 1

        right += 1

        while match_cnt == need_cnt:
            if right - left < min_len:
                start = left
                min_len = right - left
            d = source[left]
            if d in word_window:
                if word_window[d] == word_candi[d]:
                    match_cnt -= 1
                word_window[d] -= 1

            left += 1
    if min_len == sys.maxsize:
        return ""

    return source[start: start+min_len]



def lines_to_list(filename):
    try:
        with open(filename) as file:
            lines = []
            count_word_candi = {}
            for line in file:
                l_word = line.strip('\n').split(' ')
                lines = lines + l_word
            return lines
    
    except IOError as err:
        print('File Error: ' + str(err))

if __name__ == "__main__":
    word_list1 = lines_to_list("beam_birth.txt")
    res = check_sub_string(word_list1, ['hello', 'world', 'beam'])
    print(res)