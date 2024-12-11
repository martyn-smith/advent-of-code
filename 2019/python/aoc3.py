import matplotlib.pyplot as plt


def parse_locations(moves):
    def next_location(prev_location, direction, magnitude):
        moves = {
            "R": (0, magnitude),
            "L": (0, -magnitude),
            "U": (magnitude, 0),
            "D": (-magnitude, 0),
        }
        return tuple(sum(x) for x in zip(prev_location, moves[direction]))

    path = [(0, 0)]  # x,y
    for move in moves:
        direction, magnitude = move[0], int(move[1:])
        path.append(next_location(path[-1], direction, magnitude))
    return path


with open("../data/3.txt") as f:
    lines = f.readlines()
    wire_0, wire_1 = (
        [s.strip("\n") for s in lines[0].split(",")],
        [s.strip("\n") for s in lines[1].split(",")],
    )
    wire_0, wire_1 = parse_locations(wire_0)[1:], parse_locations(wire_1)[1:]
    wire_0.insert(0, [0, 0])
    wire_1.insert(0, [0, 0])

# wire 0 going down
plt.plot([w[0] for w in wire_0], [w[1] for w in wire_0], label="wire 0")
plt.plot([w[0] for w in wire_1], [w[1] for w in wire_1], label="wire 1")
plt.legend()
plt.show()
