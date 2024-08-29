"""
Advent of code day 5: boarding a plane, in a binary fashion.
"""


def get_id(p: str) -> int:
    """
    gets a numerical id from binary string.
    """
    rows = 128
    row = 0
    columns = 8
    column = 0

    for i, c in enumerate(p[:7]):
        rows >>= 1
        row += rows if c == "B" else 0
    for i, c in enumerate(p[7:]):
        columns >>= 1
        column += columns if c == "R" else 0
    return (row * 8) + column


# setup
with open("data/5.txt") as f:
    lines = f.readlines()
    IDs = [get_id(p) for p in lines]
    IDs.sort()


def part_1():
    return IDs[-1]


def part_2():
    for i, _ in enumerate(IDs[1:-1]):
        if IDs[i] - IDs[i - 1] > 1:
            return IDs[i] - 1


if __name__ == "__main__":
    print(part_1())
    print(part_2())
