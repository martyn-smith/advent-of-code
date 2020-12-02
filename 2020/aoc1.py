target = 2020

with open("1.txt") as f:
    entries = [int(l) for l in f.readlines()]

"""
There's a faster way to do this (two-sum, as in leetcode, which involves constructing a dict) -
but there's only 200 entries here and I've got other things to do today, so brute-force it is.
Due to the sliding approach, it's not quite n**2?
"""

#part 1
for i, entry in enumerate(entries):
    for complement in enumerate(entries[i:]):
        if entry + complement == target:
            print(f"{entry * complement}")
            exit()

#time: 57 ms. (On the tested platform the python startup time is about 50)

#part 2
for i, entry in enumerate(entries):
    for j, complement_1 in enumerate(entries[i:]):
        for complement_2 in enumerate(entries[i+j:]):
            if entry + complement_1 + complement_2 == target:
                print(f"{entry * complement}")
                exit()
