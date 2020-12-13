from math import prod
from itertools import permutations

with open("13.txt") as f:
    arrival = int(f.readline())
    buses = [(i, int(j)) for i, j in enumerate(f.readline().split(",")) if j != "x"]
    ids, buses = [b[0] for b in buses], [b[1] for b in buses]

def get_wait_times(arrival):
    return [bus - arrival % bus for bus in buses]

def chinese_remainder():
    times = [b - i for b, i in zip(buses, ids)]
    B = prod(buses)
    y = [B // b for b in buses]
    z = [pow(i, -1, b) for i, b in zip(y, buses)]
    return sum(prod(x) for x in zip(times, y, z)) % B

#part 1
w = min(zip(buses, get_wait_times(arrival)), key = lambda x: x[1])
print(w[0] * w[1])

#part 2
if any(b % c == 0 and b != c for b, c in permutations(buses, 2)):
    print("not coprime! Quitting...")
    exit(1)

print(chinese_remainder())