input_string= input("Pls input a string: ")
char_freq = {}

for c in input_string:
    if c in char_freq:
        char_freq[c] += 1
    else:
        char_freq[c] = 1

print(char_freq)