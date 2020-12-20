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

    def corner_tiles(self):
        return [t for t in self.tiles if t.matching_edges(self.tiles) == 2]

    def hunt(self):
        tile = self.corner_tiles()[0]
        row = 0
        checked = False
        self.grid.append([tile])
        while self.tiles != []:
            try:
                self.tiles.remove(tile)
            except ValueError:
                print(f"{tile.ID}: already removed")
            print([t.ID for t in self.grid[row]])
            try:
                print(f"checking right... {tile.ID}")
                next_tile = next(test_tile for test_tile in self.tiles 
                                    if tile.right_edge in test_tile.all_edges 
                                    or tile.right_edge[::-1] in test_tile.all_edges)
                print(f"found {next_tile.ID}")
                next_tile.find_transform(tile.right_edge, 3)
                tile = next_tile
                self.grid[row] += [tile]
            except StopIteration:
                print(f"{tile.ID} has no rightmost match")
                tile = self.grid[-1][0]
                print(f"checking down... {tile.ID}")
                try:
                    next_tile = next(test_tile for test_tile in self.tiles 
                                        if tile.bottom_edge in test_tile.all_edges 
                                        or tile.bottom_edge[::-1] in test_tile.all_edges)
                    print(f"found {next_tile.ID}")
                    print(next_tile)
                    print("===========")
                    next_tile.find_transform(tile.bottom_edge, 0)
                    if next_tile.ID == "1117":
                        print(next_tile)
                    tile = next_tile
                    self.grid.append([])
                    row += 1
                    self.grid[row] += [tile]
                    
                except StopIteration:
                    print("ended")
                    continue
        # for row in self.grid:
        #     print("========================\n")
        #     print([tile for tile in row])


    def __repr__(self):
        image = ""
        for row in self.grid:
            #image += "\n"
            for i in range(1, len(row[0].tile)-1):
                for tile in row:
                    image += tile.tile[i][1:-1]
                image += "\n"
        return image


class Tile:

    def __init__(self, pre_tile):
        self.ID = pre_tile.split("\n")[0][5:9]
        self.tile = pre_tile.split("\n")[1:]

    def __repr__(self):
        return self.body
        #return '\n'.join(self.tile)

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

    def flip_vertical(self):
        self.tile = self.tile[::-1]

    def flip_horizontal(self):
        self.tile = [l[::-1] for l in self.tile]

    def rotate(self, r):
        for _ in range(r):
            self.tile = [''.join([t[i] for t in self.tile][::-1]) for i in range(len(self.tile[0]))]

    def find_transform(self, edge, idx):
        found = (True if edge == self.all_edges[idx] else False)
        if not found:
            for i in range(4):
                self.rotate(1)
                if edge == self.all_edges[idx]:
                    found = True
                    break
        if not found:
            self.flip_horizontal()
            for _ in range(4):
                self.rotate(1)
                if edge == self.all_edges[idx]:
                    found = True
                    break

"""
Dimension checks:
144 is a 12x12 grid. That means 4 corner tiles, 40 edge tiles, 100 body tiles.
"""

#part 1
im = Image()
print(prod(int(c.ID) for c in im.corner_tiles()))


#part 2
im.hunt()
with open("20.out.txt", "w") as f:
    f.write(im.__repr__())
