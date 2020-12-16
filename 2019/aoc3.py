#import matplotlib.pyplot as plt

def parse_locations(moves):
    def next_location(prev_location, direction, magnitude):
        moves = {
            "R" : (0, magnitude),
            "L" : (0, -magnitude),
            "U" : (magnitude, 0),
            "D" : (-magnitude, 0)
        }
        return tuple(sum(x) for x in zip(prev_location, moves[direction]))

    path = [(0,0)] #x,y
    for move in moves:
        direction, magnitude = move[0], int(move[1:])
        path.append(next_location(path[-1], direction, magnitude))
    return path

with open("./3.txt") as f:
    lines = f.readlines()
wire_0, wire_1 = ([s.strip("\n") for s in lines[0].split(",")], 
                  [s.strip("\n") for s in lines[1].split(",")])
wire_0, wire_1 = parse_locations(wire_0)[1:], parse_locations(wire_1)[1:]

def intersection(point_0, point_1):
    point_0, point_1 = list(point_0), list(point_1)
    if point_0[0][0] == point_0[1][0]:
        pass
    elif point_0[0][1] == point_0[1][1]:
        pass

for point in wire_1:
    if point in wire_0:
        






