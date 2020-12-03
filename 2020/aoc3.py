from math import prod

with open("3.txt") as f, open("3_c.txt", "w+") as g:
    lines = f.readlines()
    depth = len(lines) - 1
    width = len(lines[0]) - 1

def total_trees(move: (int, int)) -> int:
    #row-major
    cursor = [0,0]
    trees = 0
    while cursor[0] <= depth:
        if lines[cursor[0]][cursor[1]] == "#":
            trees += 1
        cursor[0] += move[0]
        cursor[1] = (cursor[1] + move[1]) % width
    return trees

print(total_trees((1,3)))
moves = [(1,1), (1,3), (1,5), (1,7), (2,1)]
print(prod(total_trees(mv) for mv in moves))