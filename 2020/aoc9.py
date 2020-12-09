from itertools import combinations

preamble_len = 25
with open("9.txt") as f:
    lines = [int(l) for l in f.readlines()]

for i, number in enumerate(lines[preamble_len:]):
    found = False
    i += preamble_len
    #very similar to part 1... but since it's only 25, I'm just brute-forcing with itertools.
    for j, k in combinations(lines[i-preamble_len:i], 2):
        if j != k and j + k == number:
            found = True
            break
    if not found:
        print(f"{number}")
        break

invalid_num = 393911906 #part 1 answer

i, j = 0, 1
while sum(l for l in lines[i:j]) != invalid_num:
    if sum(l for l in lines[i:j]) < invalid_num:
        j += 1
    else:
        i += 1
print(min(lines[i:j]) + max(lines[i:j]))

