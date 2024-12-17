import re

with open("../data/3.txt", "r") as f:
    text = f.read()

def part_1():
    pairs = re.findall(r"mul\((\d+),(\d+)\)", text)
    return sum(int(p[0]) * int(p[1]) for p in pairs)

def is_excluded(p, off, on):
    try:
        closest_off = max(o for o in off if o < p)
    except ValueError:
        return False
    closest_on = max(o for o in on if o < p)
    return closest_off > closest_on

# >41423489
def part_2():
    pairs = [(m.start(), [int(g) for g in m.groups()]) for m in re.finditer(r"mul\((\d+),(\d+)\)", text)]
    off = [m.start() for m in re.finditer(r"don't\(\)", text)]
    on = [m.start() for m in re.finditer(r"do\(\)", text)]
    exclude =[(i, j) for i, j in zip(off, on)]
    pairs = [p[1] for p in pairs if not is_excluded(p[0], off, on)]
    return sum(int(p[0]) * int(p[1]) for p in pairs)

if __name__ == "__main__":
    print(part_1())
    print(part_2())
