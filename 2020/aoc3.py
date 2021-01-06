"""
Advent of code day 3: we're going tobogganing!
"""
from math import prod

def total_trees(move: (int, int)) -> int:
    """
    Counts the trees collided with in a modular grid traversal.
    """
    cursor = [0,0]
    trees = 0
    while cursor[0] <= depth:
        if lines[cursor[0]][cursor[1]] == "#":
            trees += 1
        cursor[0] += move[0]
        cursor[1] = (cursor[1] + move[1]) % width
    return trees

#setup
with open("3.txt") as f:
    lines = f.readlines()
    depth = len(lines) - 1
    width = len(lines[0]) - 1

#part 1
print(total_trees((1,3)))

#part 2
moves = [(1,1), (1,3), (1,5), (1,7), (2,1)]
print(prod(total_trees(mv) for mv in moves))