"""
Advent of code day 17: solving the world's energy crisis with Conway.
During a cycle, all cubes simultaneously change their state according to the following rules:
"""

from itertools import product

class ConwayCube:
    def __init__(self):
        with open("17.txt") as f:
            lines = f.readlines()
        self.cube =  [[line.strip("\n") for line in lines]]
        self.dimensions = {"x": len(self.cube[0][0]), "y": len(self.cube[0]), "z": len(self.cube)}

    def __repr__(self):
        ret = "" 
        for z, slice_ in enumerate(self.cube):
            ret += f"z = {z}\n"
            ret += "\n".join([row for row in slice_])
            ret += "\n\n"
        return ret

    def new_slice(self):
        return ["." * (self.dimensions["x"] + 2)] * (self.dimensions["y"] + 2)

    def expand_slice(self, slice_):
        return ["." * (self.dimensions["x"] + 2)] + [("." + s + ".") for s in slice_] + ["." * (self.dimensions["x"] + 2)]

    def step(self):
        #generate the new cube as the old one - then ignore the extremities when iterating as we know they're empty.
        new_cube = [self.new_slice()]
        for slice_ in self.cube:
            new_cube.append(self.expand_slice(slice_))
        new_cube.append(self.new_slice())
        to_activate = []
        to_deactivate = []
        
        for z in range(self.dimensions["z"]+2): #0 to old_max_z
            for y in range(self.dimensions["y"]+2):
                for x in range(self.dimensions["x"]+2):
                    """
                    If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
                    If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
                    """

                    active_count = sum(1 for neighbours in product((x-1, x, x+1), (y-1, y, y+1), (z-1, z, z+1))
                                          if not all((neighbours[0] == x, 
                                                      neighbours[1] == y, 
                                                      neighbours[2] == z))
                                          and not any((neighbours[0] not in range(self.dimensions["x"] + 2),
                                                       neighbours[1] not in range(self.dimensions["y"] + 2),
                                                       neighbours[2] not in range(self.dimensions["z"] + 2)))
                                          and new_cube[neighbours[2]][neighbours[1]][neighbours[0]] == "#")
                    if active_count == 3 and new_cube[z][y][x] == ".":
                        to_activate.append((x,y,z))
                    elif (active_count < 2 or active_count > 3) and new_cube[z][y][x] == "#":
                        to_deactivate.append((x,y,z)) 
        for a in to_activate:
            new_cube[a[2]][a[1]] = new_cube[a[2]][a[1]][:a[0]] + "#" + new_cube[a[2]][a[1]][a[0]+1:]
        for d in to_deactivate:
            new_cube[d[2]][d[1]] = new_cube[d[2]][d[1]][:d[0]] + "." + new_cube[d[2]][d[1]][d[0]+1:]
        self.cube = new_cube
        self.dimensions["x"] += 2
        self.dimensions["y"] += 2
        self.dimensions["z"] += 2

    def count_actives(self):
        active_count = 0
        for slice_ in self.cube:
            for row in slice_:
                for cell in row:
                    if cell == "#":
                        active_count += 1
        return active_count

#part 1
c = ConwayCube()
print(c)
for i in range(6):
    c.step()
    print(f"after {i+1} cycle(s):")
    print(c)
print(c.count_actives())

#240, 1180