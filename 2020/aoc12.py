"""
Advent of code day 12: steering a ferry.
"""
from itertools import cycle

directions = {
    "N" : (0,1),
    "S" : (0,-1),
    "E" : (1,0),
    "W" : (-1,0)
}

with open("12.txt") as f:
        lines = f.readlines()

def move() -> (int, int):
    """
    moves the ferry.
    """
    pos = [0,0]
    direction = "E"
    rotations = {"L": -1, "R": 1}
    rotation_order = ["N", "E", "S", "W"]

    for line in lines:
        action, magnitude = line[0], int(line[1:])
        if action in directions:
            sgn = directions[action]
            pos = [p + (s*m) for p, s, m in zip(pos, sgn, cycle([magnitude]))]
        elif action in rotations:
            magnitude //= 90
            sgn = rotations[action]
            direction = rotation_order[(rotation_order.index(direction) + (sgn * magnitude)) % 4]
        elif action == "F":
            sgn = directions[direction]
            pos = [p + (s*m) for p, s, m in zip(pos, sgn, cycle([magnitude]))]
        else:
            pass
        yield pos

def move_with_waypoint() -> (int, int):
    """
    moves the waypoint around the ferry, and occasionally the ferry itself.
    """
    ship_pos = [0,0]
    waypoint_pos = [10,1]
    waypoint_direction = "E"
    rotations = {"L": 1, "R": -1}

    for line in lines:
        action, magnitude = line[0], int(line[1:])
        if action in directions:
            sgn = directions[action]
            waypoint_pos = [p + (s*m) for p, s, m in zip(waypoint_pos, sgn, cycle([magnitude]))]
        elif action in rotations:
            distance = [(w - p) for w, p in zip(waypoint_pos, ship_pos)]
            distance = complex(distance[0], distance[1])
            sgn = rotations[action]
            distance *= complex(0, sgn) ** (magnitude // 90)
            distance = int(distance.real), int(distance.imag)
            waypoint_pos = [(p + d) for p, d in zip(ship_pos, distance)]
        elif action == "F":
            distance = [magnitude * (w - p) for w, p in zip(waypoint_pos, ship_pos)]
            ship_pos = [p + m for p, m in zip(ship_pos, distance)]
            waypoint_pos = [w + m for w, m in zip(waypoint_pos, distance)]
        else:
            pass
        yield ship_pos

#part 1
pos = [p for p in move()][-1]
print(sum(abs(p) for p in pos))

#part 2
pos = [p for p in move_with_waypoint()][-1]
print(sum(abs(p) for p in pos))