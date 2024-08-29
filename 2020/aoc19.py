"""
Advent of code day 19: validating messages with a regex builder.
"""

import re
from functools import lru_cache

# Regexes. luckily, all rules fall into these categories.
base_rule = re.compile(r"(\d+): \"(\w)\"")
compound_rule = re.compile(r"^(\d+): (\d+) (\d+) \| (\d+) (\d+)$")
sequence_rule = re.compile(r"^(\d+): (\d+) (\d+)$")
lone_rule = re.compile(r"^(\d+): (\d+)$")
lone_compound_rule = re.compile(r"^(\d+): (\d+) \| (\d+)$")
# for part 2
lone_rec_rule = re.compile(r"^(\d+): (\d+) \| (\d+) (\d+)$")
sequence_rec_rule = re.compile(r"^(\d+): (\d+) (\d+) \| (\d+) (\d+) (\d+)$")


# @lru_cache(None)
def builder(rules, root="0") -> str:
    """
    Recursively builds a string for a regex.
    (Note: for part 1 lru_cache can be enabled.)
    """
    this_rule = re.compile(f"^{root}:.*")
    rule = next(r for r in rules if this_rule.match(r))
    if b := base_rule.match(rule):
        return b.group(2)
    elif lr := lone_rec_rule.match(rule):
        return builder(rules, lr.group(2)) + "+"
    elif l := lone_rule.match(rule):
        return builder(rules, l.group(2))
    elif sr := sequence_rec_rule.match(rule):
        return (
            "("
            + "|".join(
                [
                    "(("
                    + builder(rules, sr.group(2))
                    + "){"
                    + str(i)
                    + "}("
                    + builder(rules, sr.group(3))
                    + "){"
                    + str(i)
                    + "})"
                    for i in range(1, 8)
                ]
            )
            + ")"
        )
    elif s := sequence_rule.match(rule):
        return builder(rules, s.group(2)) + builder(rules, s.group(3))
    elif lc := lone_compound_rule.match(rule):
        return (
            "(" + builder(rules, lc.group(2)) + "|" + builder(rules, lc.group(3)) + ")"
        )
    elif c := compound_rule.match(rule):
        return (
            "("
            + builder(rules, c.group(2))
            + builder(rules, c.group(3))
            + "|"
            + builder(rules, c.group(4))
            + builder(rules, c.group(5))
            + ")"
        )
    print(f"{rule} matches nothing!")


def parse_input():
    with open("data/19.txt") as f:
        rules, messages = f.read().split("\n\n")
        rules = rules.split("\n")
        messages = messages.split("\n")
        return rules, messages


# b_count = sum(1 for r in rules if base_rule.match(r))
# c_count = sum(1 for r in rules if compound_rule.match(r))
# s_count = sum(1 for r in rules if sequence_rule.match(r))
# l_count = sum(1 for r in rules if lone_rule.match(r))
# lc_count = sum(1 for r in rules if lone_compound_rule.match(r))


def part_1():
    rules, messages = parse_input()
    b = "^" + builder(rules) + "$"
    total_rule = re.compile(b)
    return sum(1 for m in messages if total_rule.match(m))


def part_2():
    rules, messages = parse_input()
    i = rules.index("8: 42")
    rules = rules[:i] + ["8: 42 | 42 8"] + rules[i:]
    i = rules.index("11: 42 31")
    rules = rules[:i] + ["11: 42 31 | 42 11 31"] + rules[1:]
    b = "^" + builder(rules) + "$"
    total_rule = re.compile(b)
    return sum(1 for m in messages if total_rule.match(m))


if __name__ == "__main__":
    print(part_1())
    print(part_2())
