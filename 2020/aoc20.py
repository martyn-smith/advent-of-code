"""
Advent of code day 20: rotating images to find Nessy.
"""
from math import prod

sea_monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
"""

class Image:

    def __init__(self):
        with open("20.txt") as f:
            pre_tiles = f.read().split("\n\n")
        self.tiles = [Tile(p) for p in pre_tiles]
        self.grid = []

    @property
    def corner_tiles(self):
        return [t for t in self.tiles if t.matching_edges(self.tiles) == 2]

    def solve(self):
        tile = self.corner_tiles[0]
        row = 0
        checked = False
        self.grid.append([tile])
        while self.tiles != []:
            self.tiles.remove(tile)
            try:
                #print(f"checking right... {tile.ID}")
                next_tile = next(test_tile for test_tile in self.tiles 
                                    if tile.right_edge in test_tile.all_edges 
                                    or tile.right_edge[::-1] in test_tile.all_edges)
                #print(f"found {next_tile.ID}")
                next_tile.find_transform(tile.right_edge, "left")
                tile = next_tile
                self.grid[row] += [tile]
            except StopIteration:
                tile = self.grid[-1][0]
                try:
                    #print(f"checking down... {tile.ID}")
                    next_tile = next(test_tile for test_tile in self.tiles 
                                        if tile.bottom_edge in test_tile.all_edges 
                                        or tile.bottom_edge[::-1] in test_tile.all_edges)
                    #print(f"found {next_tile.ID}")
                    next_tile.find_transform(tile.bottom_edge, "top")
                    tile = next_tile
                    self.grid.append([])
                    row += 1
                    self.grid[row] += [tile]
                except StopIteration:
                    continue

    def __repr__(self):
        image = ""
        for row in self.grid:
            for i in range(1, len(row[0].tile)-1):
                for tile in row:
                    image += tile.tile[i][1:-1]
                image += "\n"
        #image = ""
        # for row in self.grid:
        #     for tile in row:
        #         image = '\n'.join([i + t[1:-1] for i, t in zip(image.split("\n"), repr(tile).split("\n"))])
        #         image += "\n"
        return image

    def hunt(self, monster):
        monster = [m for m in monster.strip("\n").split("\n")]
        grid = repr(self).strip("\n").split("\n")
        monster_count = 0
        for r in range(8):
            if r == 4:
                grid = flip_horizontal(grid)
            grid = rot90(grid)
            for y in range(len(grid) - len(monster)):
                for x in range(len(grid[0]) - len(monster[0])):
                    candidate_monster = ["".join("#" if grid[y+i][x+j] == "#" 
                                                        and monster[i][j] == "#" 
                                                        else " " 
                                                    for j in range(len(monster[0]))) 
                                                for i in range(len(monster))]
                    #print("\n".join(m + "\t" + c for m, c in zip(monster, candidate_monster)))
                    if candidate_monster == monster:
                        #print(f"found match! r = {r}")
                        monster_count += 1
        return monster_count

class Tile:

    def __init__(self, pre_tile):
        self.ID = pre_tile.split("\n")[0][5:9]
        self.tile = pre_tile.split("\n")[1:]

    def __repr__(self):
        return self.body

    @property
    def top_edge(self):
        return self.tile[0]

    @property
    def bottom_edge(self):
        return self.tile[-1]

    @property
    def left_edge(self):
        return ''.join([t[0] for t in self.tile])

    @property
    def right_edge(self):
        return ''.join([t[-1] for t in self.tile])

    @property
    def all_edges(self):
        return [self.top_edge] + [self.right_edge] + [self.bottom_edge] + [self.left_edge]

    @property
    def body(self):
        return '\n'.join([t[1:-1] for t in self.tile[1:-1]])

    @property
    def edge_enum(self):
        return ["top", "right", "bottom", "left"]

    def matching_edges(self, tiles):
        tiles = [t for t in tiles if t.ID != self.ID]
        top_match = any(self.top_edge in t.all_edges or self.top_edge[::-1] in t.all_edges for t in tiles)
        right_match = any(self.right_edge in t.all_edges or self.right_edge[::-1] in t.all_edges for t in tiles)
        bottom_match = any(self.bottom_edge in t.all_edges or self.bottom_edge[::-1] in t.all_edges for t in tiles)
        left_match = any(self.left_edge in t.all_edges or self.left_edge[::-1] in t.all_edges for t in tiles)
        return sum((top_match, bottom_match, left_match, right_match))

    def find_transform(self, edge, seek):
        idx = self.edge_enum.index(seek)
        for r in range(8):
            if r == 4:
                self.tile = flip_horizontal(self.tile)
            self.tile = rot90(self.tile)
            if edge == self.all_edges[idx]:
                break


def rot90(tile):
    return [''.join([t[i] for t in tile][::-1]) for i in range(len(tile[0]))]

def flip_vertical(tile):
        return tile[::-1]

def flip_horizontal(tile):
        return [l[::-1] for l in tile]

#part 1
im = Image()
print(prod(int(c.ID) for c in im.corner_tiles))

#part 2
im.solve()
monster_count = im.hunt(sea_monster)
monster_tiles = monster_count * sum(1 for i in sea_monster if i == "#")
print(sum(1 for i in repr(im) if i == "#") - monster_tiles)
