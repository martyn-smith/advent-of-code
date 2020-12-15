with open("1.txt") as f:
    entries = [int(l) for l in f.readlines()]

#part 1
target = 2020
complements = {}
for entry in entries:
    if complements.get(target - entry):
        print(f"{entry * (target - entry)}")
    else:
        complements[entry] = True

#part 2
for i, entry in enumerate(entries):
    for j, complement_1 in enumerate(entries[i:]):
        for complement_2 in entries[i+j:]:
            if entry + complement_1 + complement_2 == target:
                print(f"{entry * complement_1 * complement_2}")
