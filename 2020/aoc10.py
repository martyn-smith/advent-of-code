"""
Advent of code day 10: playing with voltage.
"""
from functools import lru_cache

@lru_cache(None)
def find_valid_paths(jolt: list) -> int:
    """
    dynamic/recursive cached path-finding solution.
    """
    if jolt == jolts[-1]:
        return 1
    return sum(find_valid_paths(jolt + i + 1) for i in range(3) if jolt + i + 1 in jolts)
#setup
with open("10.txt") as f:
    lines = f.readlines()

jolts = [int(l) for l in lines]
jolts.sort()
jolts.insert(0, 0)
jolts.append(max(jolts) + 3)

#part 1
deltas = [i - j for i, j in zip(jolts[1:], jolts[:-1])]
print(deltas.count(1) * deltas.count(3))

#part 2
print(find_valid_paths(jolts[0]))