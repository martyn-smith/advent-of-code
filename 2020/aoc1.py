"""
Advent of code 2020 day 1: we're fudging expense reports.
"""
with open("1.txt") as f:
    entries = [int(l) for l in f.readlines()]

target = 2020

def find_pair(target: int) -> (int, int):
    """
    Finds two entries that sum to target. Set-based implementatino.
    """
    complements = set()
    for entry in entries:
        if (target - entry) in complements:
            return (entry,  target - entry)
        else:
            complements.add(entry)

def find_triad(target: int) -> (int, int, int):
    """
    Finds three entries that sum to target. Brute-force implementation.
    """
    for i, entry in enumerate(entries):
        for j, complement_1 in enumerate(entries[i:]):
            for complement_2 in entries[i+j:]:
                if entry + complement_1 + complement_2 == target:
                    return (entry, complement_1, complement_2)

#part 1
a, b = find_pair(target)
print(a * b)

#part 2
a, b, c = find_triad(target)
print(a * b * c)
