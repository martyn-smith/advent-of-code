def get_id(p: str) -> int:
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

with open("5.txt") as f:
    IDs = [get_id(p) for p in f.readlines()]

IDs.sort()
print(IDs[-1])
for i, _ in enumerate(IDs[1:-1]):
    if IDs[i] - IDs[i-1] > 1:
        print(IDs[i] - 1)

