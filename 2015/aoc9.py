from itertools import permutations
from random import choice
from copy import deepcopy

towns = set()
weights = {}

with open("data/9.txt") as f:
    for line in f.readlines():
        start_town = line.split(" to ")[0]
        end_town = line.split(" to ")[1].split(" = ")[0]
        weight = int(line.split(" = ")[1])
        #order doesn't matter here, an edge is an edge
        j = [start_town, end_town]
        j.sort()
        weights[(j[0], j[1])] = weight
        towns.add(start_town)
        towns.add(end_town)
#print(f"{weights=}")

paths = []
def traverse(start, path, remaining):
    global paths
    remaining.remove(start)
    #print(f"{start=}\t{remaining=}")
    if len(remaining) == 0:
        #IRDGI, but if you append the actual PATH here,
        #it bugs out (wtaf?)
        paths.append(sum(r for r in path.values()))
    for option in remaining:
        try:
            #order DOES matter here,
            j = [start, option]
            j.sort()
            weight = weights[(j[0], j[1])]
            #print(f"{start} to {option} = {weight}")
            path[option] = weight
            traverse(option, path, deepcopy(remaining))
        except KeyError:
            print(f"no path between {start} and {option}")
            pass

def part_1():
    for town in towns:
        traverse(town, {town:0}, deepcopy(towns))
    #print(paths)
    return min(paths)

def part_2():
    for town in towns:
        traverse(town, {town:0}, deepcopy(towns))
    #print(paths)
    return max(paths)


if __name__ == "__main__":
    print(part_1())
    print(part_2())
