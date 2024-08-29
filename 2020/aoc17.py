"""
Advent of code day 17: solving the world's energy crisis with Conway.

Hashmap-based implementation.
"""

from itertools import product


class ConwayCube:
    def __init__(self, dimensionality=3):
        self.actives = {}
        self.dimensionality = dimensionality
        with open("data/17.txt") as f:
            lines = f.readlines()
        for y, line in enumerate(lines):
            for x, char in enumerate(line):
                if char == "#":
                    key = tuple([x, y] + [0] * (self.dimensionality - 2))
                    self.actives[key] = True

    def step(self):
        to_activate = []
        to_deactivate = []
        for point in self.actives:
            ranges = [(r - 1, r, r + 1) for r in point]
            search_space = [p for p in product(*ranges) if p != point]
            if sum(1 for s in search_space if s in self.actives) not in (2, 3):
                to_deactivate.append(point)
            for search_point in search_space:
                ranges_2 = [(r - 1, r, r + 1) for r in search_point]
                search_space_2 = [p for p in product(*ranges_2) if p != search_point]
                if sum(1 for s in search_space_2 if s in self.actives) == 3:
                    if search_point not in to_activate:
                        to_activate.append(search_point)
        for a in to_activate:
            self.actives[a] = True
        for d in to_deactivate:
            self.actives.pop(d)


def part_1():
    c = ConwayCube()
    for i in range(6):
        c.step()
    return len(c.actives)


def part_2():
    c = ConwayCube(4)
    for i in range(6):
        c.step()
    return len(c.actives)


if __name__ == "__main__":
    print(part_1())
    print(part_2())
