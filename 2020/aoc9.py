"""
Advent of code day 9: Breaking the in-flight entertainment's copy-protection.
"""

from itertools import combinations

preamble_len = 25


def find_invalid_XMAS_number() -> int:
    """
    Finds the first number not expressible is the sum of two of the 25 preceding numbers.
    """
    for i, number in enumerate(lines[preamble_len:]):
        found = False
        i += preamble_len
        for j, k in combinations(lines[i - preamble_len : i], 2):
            if j != k and j + k == number:
                found = True
                break
        if not found:
            return number


def find_contiguous_set(invalid_num: int) -> (int, int):
    """
    Finds the extrema of the contiguous set that sums to invalid_num
    """
    i, j = 0, 1
    while sum(l for l in lines[i:j]) != invalid_num:
        if sum(l for l in lines[i:j]) < invalid_num:
            j += 1
        else:
            i += 1
    return i, j


# setup
with open("data/9.txt") as f:
    lines = [int(l) for l in f.readlines()]


def part_1():
    invalid_num = find_invalid_XMAS_number()
    return invalid_num


def part_2():
    invalid_num = part_1()
    i, j = find_contiguous_set(invalid_num)
    return min(lines[i:j]) + max(lines[i:j])


if __name__ == "__main__":
    print(part_1())
    print(part_2())
