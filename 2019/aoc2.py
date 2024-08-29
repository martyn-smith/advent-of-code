from itertools import product


def gravity_assist(noun, verb):
    def add(a, b, write_idx):
        # print(f"writing {a} + {b} to pos {write_idx}")
        codes[write_idx] = codes[a] + codes[b]

    def multiply(a, b, write_idx):
        # print(f"writing {a} * {b} to pos {write_idx}")
        codes[write_idx] = codes[a] * codes[b]

    def terminate(_, __, ___):
        pass

    opcodes = {1: add, 2: multiply, 99: terminate}

    with open("data/2.txt") as f:
        codes = [int(c) for c in f.read().strip("\n").split(",")]
        codes[1], codes[2] = noun, verb

    i = 0
    while True:
        # print(codes[i])
        op = opcodes[codes[i]]
        if op == terminate:
            return codes[0]
        op(codes[i + 1], codes[i + 2], codes[i + 3])
        i += 4


def hunt(target):
    for n, v in product(range(100), range(100)):
        if gravity_assist(n, v) == target:
            return n, v
    raise StopIteration


def part_1():
    return gravity_assist(12, 2)


def part_2():
    n, v = hunt(19690720)
    return (100 * n) + v


if __name__ == "__main__":
    print(part_1())
    print(part_2())
