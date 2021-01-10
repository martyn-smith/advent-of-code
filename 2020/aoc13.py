"""
Advent of code day 13: playing with buses and modular arithmetic.
"""
from math import prod
from itertools import permutations

def get_wait_times(arrival: int) -> list:
    """
    Self-explanatory, returns the wait time for each bus.
    """
    return [bus - arrival % bus for bus in buses]

def incremental_prime_search() -> int:
    """
    A fast prime-based search that solves the chinese remainder problem.
    """
    delta = 1
    x = 1
    times = [(b - i) % b for b, i in zip(buses, ids)]
    for t, b in zip(times, buses):
        while x % b != t:
            x += delta
        delta *= b
    return x

def chinese_remainder() -> int:
    """
    The 'proper' approach. Currently unused.
    """
    times = [b - i for b, i in zip(buses, ids)]
    B = prod(buses)
    y = [B // b for b in buses]
    z = [pow(i, -1, b) for i, b in zip(y, buses)]
    return sum(prod(x) for x in zip(times, y, z)) % B

#setup
with open("data/13.txt") as f:
    arrival = int(f.readline())
    buses = [(i, int(j)) for i, j in enumerate(f.readline().split(",")) if j != "x"]
    ids, buses = [b[0] for b in buses], [b[1] for b in buses]

def part_1():
    w = min(zip(buses, get_wait_times(arrival)), key = lambda x: x[1])
    return w[0] * w[1]

def part_2():
    if any(b % c == 0 and b != c for b, c in permutations(buses, 2)):
        print("not coprime! Quitting...")
        exit(1)
    return incremental_prime_search()

if __name__ == "__main__":
    print(part_1())
    print(part_2())