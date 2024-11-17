"""
Advent of Code day 5: naughty/nice strings
"""

import re

vowels = re.compile(r"([aeiou].*){3}")
repeat = re.compile(r"(\w)\1")
pairs = re.compile(r"(ab)|(cd)|(pq)|(xy)")
non_overlap = re.compile(r"(\w{2}).*\1")
sandwich = re.compile(r"(\w)[^\1]\1")


def nice_or_naughty(line):
    return vowels.search(line) and repeat.search(line) and not pairs.search(line)


def nice_or_naughty_2(line):
    return non_overlap.search(line) and sandwich.search(line)


with open("../data/5.txt") as f:
    lines = f.readlines()


def part_1():
    return sum(1 for line in lines if nice_or_naughty(line))


def part_2():
    return sum(1 for line in lines if nice_or_naughty_2(line))


if __name__ == "__main__":
    print(part_1())
    print(part_2())
