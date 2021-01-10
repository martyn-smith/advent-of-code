"""
Advent of code day 24: non-orthogonal vector spaces and game of life,
but no penrose tiles and no crabs.
"""
import re

#not actually used, but a useful illustation so I'm keeping it.
directions = {
    "e" : (1,0),
    "w" : (-1,0),
    "se" : (0,1),
    "nw" : (0,-1),
    "sw" : (-1,1),
    "ne" : (1,-1)
}

east = re.compile("(?<!n|s)e")
west = re.compile("(?<!n|s)w")
southwest = re.compile("sw")
northeast = re.compile("ne")
southeast = re.compile("se")
northwest = re.compile("nw")

def build(lines) -> list:
    positions = []
    for line in lines:
        east_count = len(east.findall(line)) - len(west.findall(line))
        southeast_count = len(southeast.findall(line)) - len(northwest.findall(line))
        southwest_count = len(southwest.findall(line)) - len(northeast.findall(line))
        east_count -= southwest_count
        southeast_count += southwest_count
        position = (east_count, southeast_count)
        #turns out the easiest way to ensure only an odd number of repititions make it to the list is
        #to eliminate duplicates here.
        if position in positions:
            positions.remove(position)
        else:
            positions += [position]
    return positions

def play(positions: set) -> set:
    """
    Conway again.
    Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
    Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
    """
    def get_neighbours(pos: tuple) -> list:
        return [(pos[0]+1, pos[1]), (pos[0]-1, pos[1]), 
                (pos[0], pos[1]+1), (pos[0], pos[1]-1),
                (pos[0]-1, pos[1]+1), (pos[0]+1, pos[1]-1)]
    candidates = []
    for pos in positions:
        candidates += get_neighbours(pos)
    candidates = set(candidates + list(positions))
    to_activate = []
    to_deactivate = []
    for c in candidates:
        neighbours = get_neighbours(c)
        if c in positions: 
            s = sum(1 for n in neighbours if n in positions)
            if s > 2 or s < 1:
                to_deactivate += [c]
        elif sum(1 for n in neighbours if n in positions) == 2:
            to_activate += [c]
    for position in to_deactivate:
        positions.remove(position)
    for position in to_activate:
        positions.add(position)
    return positions

#setup
with open("data/24.txt") as f:
    lines = f.readlines()

def part_1():
    positions = build(lines)
    return len(positions)

def part_2():
    positions = set(build(lines)) #necessary to prevent O(n). SPEEEEEEEEED
    for i in range(100):
        positions = play(positions)
    return len(positions)

if __name__ == "__main__":
    print(part_1())
    print(part_2())